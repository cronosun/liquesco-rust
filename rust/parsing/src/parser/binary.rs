use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::types::binary::TBinary;
use liquesco_serialization::types::binary::Binary;
use liquesco_serialization::core::Serializer;

pub struct PBinary;

impl<'a> Parser<'a> for PBinary {
    type T = TBinary<'a>;

    fn parse<'c, C>(
        _: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        _: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let binary = C::TConverter::require_binary(&value.value)?;
        Ok(Binary::serialize(writer, &binary)?)
    }
}
