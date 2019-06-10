use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::sint::TSInt;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::types::sint::SInt64;

pub struct PSInt;

impl<'a> Parser<'a> for PSInt {
    type T = TSInt<'a>;

    fn parse<'c, C>(
        _: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        _: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let value = C::TConverter::require_i64(value.as_ref())?;
        SInt64::serialize(writer, &value)?;
        Result::Ok(())
    }
}
