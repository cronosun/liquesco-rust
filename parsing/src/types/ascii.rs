use crate::converter::Converter;
use crate::core::Context;
use crate::core::Parser;
use crate::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::types::ascii::TAscii;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::types::unicode::Unicode;

pub struct PAscii;

impl<'a> Parser<'a> for PAscii {
    type T = TAscii<'a>;

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
