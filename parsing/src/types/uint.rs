use crate::converter::Converter;
use crate::core::Context;
use crate::core::Parser;
use crate::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::types::uint::TUInt;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::types::uint::UInt128;

pub struct PUInt;

impl<'a> Parser<'a> for PUInt {
    type T = TUInt<'a>;

    fn parse<'c, C>(
        _: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        _: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let value = C::TConverter::require_u128(value.as_ref())?;
        UInt128::serialize(writer, &value)?;
        Result::Ok(())
    }
}
