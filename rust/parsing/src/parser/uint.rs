use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_schema::uint::TUInt;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::uint::UInt64;
use liquesco_common::error::LqError;

pub struct PUInt;

impl Parser<'static> for PUInt {
    type T = TUInt;

    fn parse<'c, C>(
        _: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        _: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        C::TConverter::require_no_name(value)?;
        let value = C::TConverter::require_u64(value.as_ref())?;
        UInt64::serialize(writer, &value)?;
        Result::Ok(())
    }
}
