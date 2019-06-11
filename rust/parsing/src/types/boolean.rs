use crate::converter::Converter;
use crate::core::Context;
use crate::core::Parser;
use crate::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::types::boolean::TBool;
use liquesco_serialization::types::boolean::Bool;
use liquesco_serialization::core::Serializer;

pub struct PBool;

impl<'a> Parser<'a> for PBool {
    type T = TBool<'a>;

    fn parse<'c, C>(
        _: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        _: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let value = C::TConverter::require_bool(value.as_ref())?;
        Bool::serialize(writer, &value)?;
        Result::Ok(())
    }
}
