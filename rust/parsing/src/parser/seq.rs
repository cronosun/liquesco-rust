use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::Parser;
use crate::parser::identifier::PIdentifier;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::seq::TSeq;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::seq::SeqHeader;
use std::convert::TryFrom;

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
        for item in seq {
            context.parse(writer, r#type.element(), item)?;
        }
        Ok(())
    }
}
