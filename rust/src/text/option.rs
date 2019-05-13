use crate::schema::option::TOption;
use crate::serialization::option::Presence;
use crate::serialization::core::Serializer;
use crate::text::core::Context;
use crate::text::core::ParseError;
use crate::text::core::Parser;
use crate::text::value::Converter;

pub struct POption;

impl Parser<'static> for POption {
    type T = TOption;

    fn parse<C>(context: &mut C, writer : &mut C::TWriter, r#type: Self::T) -> Result<(), ParseError>
    where
        C: Context
    {        
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
