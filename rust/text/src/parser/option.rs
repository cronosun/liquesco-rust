use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_schema::option::TOption;
use liquesco_core::serialization::core::Serializer;
use liquesco_core::serialization::option::Presence;

pub struct POption;

impl Parser<'static> for POption {
    type T = TOption;

    fn parse<'c, C>(
        context: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        r#type: &Self::T,
    ) -> Result<(), ParseError>
    where
        C: Context<'c>,
    {
        C::TConverter::require_no_name(value)?;
        if value.as_ref().is_nothing() {
            Presence::serialize(writer, &Presence::Absent)?;
            Result::Ok(())
        } else {
            Presence::serialize(writer, &Presence::Present)?;
            context.parse(writer, r#type.r#type, value)
        }
    }
}
