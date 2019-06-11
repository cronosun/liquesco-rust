use crate::converter::Converter;
use crate::core::Context;
use crate::core::Parser;
use crate::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::types::sint::TSInt;
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
