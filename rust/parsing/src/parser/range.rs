use liquesco_common::error::LqError;
use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_schema::ascii::TAscii;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::unicode::Unicode;
use liquesco_schema::range::{TRange, Inclusion};
use liquesco_serialization::seq::SeqHeader;
use core::borrow::Borrow;
use std::convert::TryFrom;
use liquesco_serialization::boolean::Bool;

pub struct PRange;

impl Parser<'static> for PRange {
    type T = TRange;

    fn parse<'c, C>(
        context: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        r#type: &Self::T,
    ) -> Result<(), LqError>
        where
            C: Context<'c>,
    {
        C::TConverter::require_no_name(value)?;

        let sequence = C::TConverter::require_seq(&value.value)?;
        let supplied_inclusion = r#type.inclusion()==Inclusion::Supplied;
        let number_of_fields = if supplied_inclusion { 4 } else {2};

        if sequence.len()!=number_of_fields {
            return LqError::err_new(format!("Range parsing: Got a sequence (this is \
            correct) but the sequence has a length of {}. I need a length of {}: [start, end, \
            start included, end included] or just [start, end] (when schema provides inclusion).",
                                            sequence.len(), number_of_fields));
        }

        let u32_number_of_fields = u32::try_from(number_of_fields)?;
        SeqHeader::serialize(writer, &SeqHeader::new(u32_number_of_fields))?;

        let start = context.parse(writer, r#type.element(), &sequence[0])?;
        let end = context.parse(writer, r#type.element(), &sequence[1])?;

        if supplied_inclusion {
            let value = C::TConverter::require_bool(&sequence[2].value)?;
            Bool::serialize(writer, &value)?;
            let value = C::TConverter::require_bool(&sequence[3].value)?;
            Bool::serialize(writer, &value)?;
        }

        Ok(())
    }
}
