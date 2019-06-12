use crate::converter::Converter;
use crate::core::Context;
use crate::core::Parser;
use crate::types::identifier::PIdentifier;
use crate::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::types::seq::{TSeq, Ordering, Direction};
use liquesco_serialization::core::Serializer;
use liquesco_serialization::types::seq::SeqHeader;
use liquesco_serialization::slice_reader::SliceReader;
use std::convert::TryFrom;
use liquesco_schema::core::TypeRef;
use std::io::Write;
use liquesco_schema::core::Schema;
use std::cmp;

pub struct PSeq;

impl<'a> Parser<'a> for PSeq {
    type T = TSeq<'a>;

    fn parse<'c, C>(
        context: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        r#type: &Self::T,
    ) -> Result<(), LqError>
        where
            C: Context<'c>,
    {
        // Maybe it's a special case (seq -> ascii)
        let parsed_as_identifier = PIdentifier::maybe_parse(context, writer, value, r#type)?;
        if parsed_as_identifier {
            return Ok(());
        }

        let seq = C::TConverter::require_seq(value.as_ref())?;
        let len = seq.len();
        let u32_len = u32::try_from(len)?;
        SeqHeader::serialize(writer, &SeqHeader::new(u32_len))?;

        let (sorted, ascending) = match r#type.ordering() {
            Ordering::None => (false, false),
            Ordering::Sorted(sorted) => {
                match sorted.direction {
                    Direction::Ascending => (true, true),
                    Direction::Descending => (true, false)
                }
            }
        };

        if !sorted {
            for item in seq {
                context.parse(writer, r#type.element(), item)?;
            }
        } else {
            // sorted: in this case first parse to a vec and then sort that
            let mut elements = Vec::with_capacity(len);
            for item in seq {
                let element_vec = context.parse_to_vec( r#type.element(), item)?;
                elements.push(element_vec);
            }
            sort(context, r#type.element(), ascending, &mut elements)?;
            for element in &elements {
                writer.write_all(element)?;
            }
        }
        Ok(())
    }
}

fn sort<'c, C>(
    context: &mut C,
    element_type: &TypeRef,
    ascending: bool,
    elements : &mut Vec<Vec<u8>>) -> Result<(), LqError>
    where
        C: Context<'c>, {
    let mut error = false;

    elements.sort_by(|a, b| {
        let mut a_reader = SliceReader::from(a);
        let mut b_reader = SliceReader::from(b);

        let result = context
            .schema()
            .compare(element_type, &mut a_reader, &mut b_reader);
        if let Ok(result) = result {
            if ascending {
                result
            } else {
                result.reverse()
            }
        } else {
            error = true;
            cmp::Ordering::Equal
        }
    });

    if error {
        LqError::err_new("Unable to sort keys in the given sequence")
    } else {
        Ok(())
    }
}