use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::float::TFloat32;
use liquesco_schema::float::TFloat64;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::types::float::Float;

pub struct PFloat32;

impl<'a> Parser<'a> for PFloat32 {
    type T = TFloat32<'a>;

    fn parse<'c, C>(
        _: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        _: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let value = C::TConverter::require_f32(value.as_ref())?;
        Float::serialize(writer, &Float::F32(value))?;
        Result::Ok(())
    }
}

pub struct PFloat64;

impl<'a> Parser<'a> for PFloat64 {
    type T = TFloat64<'a>;

    fn parse<'c, C>(
        _: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        _: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let value = C::TConverter::require_f64(value.as_ref())?;
        Float::serialize(writer, &Float::F64(value))?;
        Result::Ok(())
    }
}
