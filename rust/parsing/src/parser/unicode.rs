use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::unicode::TUnicode;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::types::unicode::Unicode;

pub struct PUnicode;

impl<'a> Parser<'a> for PUnicode {
    type T = TUnicode<'a>;

    fn parse<'c, C>(
        _: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        _: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let text = C::TConverter::require_text(value.as_ref())?;
        Ok(Unicode::serialize(writer, text)?)
    }
}
