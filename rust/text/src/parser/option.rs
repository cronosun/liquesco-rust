use liquesco_core::schema::option::TOption;
use liquesco_core::serialization::option::Presence;
use liquesco_core::serialization::core::Serializer;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::value::Converter;

pub struct POption;

impl Parser<'static> for POption {
    type T = TOption;

    fn parse<'c, C>(context: &C, writer : &mut C::TWriter, r#type: &Self::T) -> Result<(), ParseError>
        where
            C: Context<'c> {
        C::TConverter::require_no_name(context.text_value())?;
        if context.value().is_none() {
            Presence::serialize(writer, &Presence::Absent)?;
            Result::Ok(())
        } else {
            Presence::serialize(writer, &Presence::Present)?;
            let value = C::TConverter::require_maybe_present(context.value())?;    
            context.parse(writer, r#type.r#type, value)
        }
    }
}
