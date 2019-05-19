use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_core::schema::sint::TSInt;
use liquesco_core::serialization::core::Serializer;
use liquesco_core::serialization::sint::SInt64;

pub struct PSInt;

impl Parser<'static> for PSInt {
    type T = TSInt;

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
        let value = C::TConverter::require_i64(value.as_ref())?;
        SInt64::serialize(writer, &value)?;
        Result::Ok(())
    }
}
