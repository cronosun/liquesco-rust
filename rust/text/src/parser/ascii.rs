use liquesco_core::serialization::unicode::Unicode;
use liquesco_core::serialization::core::Serializer;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::value::Converter;
use liquesco_core::schema::ascii::TAscii;

pub struct PAscii;

impl Parser<'static> for PAscii {
    type T = TAscii;

    fn parse<'c, C>(context: &C, writer : &mut C::TWriter, _: &Self::T) -> Result<(), ParseError>
        where
            C: Context<'c> {
        C::TConverter::require_no_name(context.text_value())?;

        let text = C::TConverter::require_text(context.value())?;
        Ok(Unicode::serialize(writer, text)?)
    }
}
