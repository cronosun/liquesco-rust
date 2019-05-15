use liquesco_core::schema::boolean::TBool;
use liquesco_core::serialization::core::Serializer;
use liquesco_core::serialization::boolean::Bool;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::converter::Converter;

pub struct PBool;

impl Parser<'static> for PBool {
    type T = TBool;

    fn parse<'c, C>(context: &C, writer : &mut C::TWriter, _: &Self::T) -> Result<(), ParseError>
        where
            C: Context<'c> {
        C::TConverter::require_no_name(context.text_value())?;
        let value = C::TConverter::require_bool(context.value())?;
        Bool::serialize(writer, &value)?;
        Result::Ok(())
    }
}
