use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_core::schema::float::TFloat32;
use liquesco_core::schema::float::TFloat64;
use liquesco_core::serialization::core::Serializer;
use liquesco_core::serialization::float::Float;

pub struct PFloat32;

impl Parser<'static> for PFloat32 {
    type T = TFloat32;

    fn parse<'c, C>(
        _: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        _: &Self::T,
    ) -> Result<(), ParseError>
    where
        C: Context<'c>,
    {
        C::TConverter::require_no_name(value)?;
        let value = C::TConverter::require_f32(value.as_ref())?;
        Float::serialize(writer, &Float::F32(value))?;
        Result::Ok(())
    }
}

pub struct PFloat64;

impl Parser<'static> for PFloat64 {
    type T = TFloat64;

    fn parse<'c, C>(
        _: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        _: &Self::T,
    ) -> Result<(), ParseError>
    where
        C: Context<'c>,
    {
        C::TConverter::require_no_name(value)?;
        let value = C::TConverter::require_f64(value.as_ref())?;
        Float::serialize(writer, &Float::F64(value))?;
        Result::Ok(())
    }
}
