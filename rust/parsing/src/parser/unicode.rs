use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_schema::unicode::TUnicode;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::unicode::Unicode;
use liquesco_common::error::LqError;

pub struct PUnicode;

impl Parser<'static> for PUnicode {
    type T = TUnicode;

    fn parse<'c, C>(
        _: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        _: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        C::TConverter::require_no_name(value)?;

        let text = C::TConverter::require_text(value.as_ref())?;
        Ok(Unicode::serialize(writer, text)?)
    }
}
