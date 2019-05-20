use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_schema::ascii::TAscii;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::unicode::Unicode;

pub struct PAscii;

impl Parser<'static> for PAscii {
    type T = TAscii;

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

        let text = C::TConverter::require_text(value.as_ref())?;
        Ok(Unicode::serialize(writer, text)?)
    }
}
