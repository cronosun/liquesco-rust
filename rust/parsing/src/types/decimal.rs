use crate::converter::Converter;
use crate::core::Context;
use crate::core::Parser;
use crate::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_serialization::core::Serializer;
use liquesco_schema::types::decimal::{TDecimal, DecimalSerialization};

pub struct PDecimal;

impl<'a> Parser<'a> for PDecimal {
    type T = TDecimal<'a>;

    fn parse<'c, C>(
        _: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        _: &Self::T,
    ) -> Result<(), LqError>
        where
            C: Context<'c>,
    {
        let decimal = C::TConverter::require_decimal(&value.value)?;
        DecimalSerialization::serialize(writer, &decimal)
    }
}
