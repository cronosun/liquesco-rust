use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_core::schema::boolean::TBool;
use liquesco_core::serialization::boolean::Bool;
use liquesco_core::serialization::core::Serializer;

pub struct PBool;

impl Parser<'static> for PBool {
    type T = TBool;

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
        let value = C::TConverter::require_bool(value.as_ref())?;
        Bool::serialize(writer, &value)?;
        Result::Ok(())
    }
}
