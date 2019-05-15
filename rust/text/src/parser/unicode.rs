use liquesco_core::serialization::unicode::Unicode;
use liquesco_core::serialization::core::Serializer;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::converter::Converter;
use liquesco_core::schema::unicode::TUnicode;

pub struct PUnicode;

impl Parser<'static> for PUnicode {
    type T = TUnicode;

    fn parse<'c, C>(context: &C, writer : &mut C::TWriter, _: &Self::T) -> Result<(), ParseError>
        where
            C: Context<'c> {
        C::TConverter::require_no_name(context.text_value())?;

        let text = C::TConverter::require_text(context.value())?;
        Ok(Unicode::serialize(writer, text)?)
    }
}
