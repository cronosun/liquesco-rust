use liquesco_core::schema::option::TOption;
use liquesco_core::serialization::option::Presence;
use liquesco_core::serialization::core::Serializer;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::value::Converter;
use liquesco_core::schema::seq::{TSeq, Ordering};
use liquesco_core::serialization::seq::SeqHeader;
use std::convert::TryFrom;
use liquesco_core::common::error::LqError;
use liquesco_core::schema::ascii::TAscii;
use liquesco_core::serialization::value::Value::Unicode;

pub struct PAscii;

impl Parser<'static> for PAscii {
    type T = TAscii;

    fn parse<'c, C>(context: &C, writer : &mut C::TWriter, r#type: &Self::T) -> Result<(), ParseError>
        where
            C: Context<'c> {
        C::TConverter::require_no_name(context.text_value())?;

        let text = C::TConverter::require_text(context.value())?;
        Ok(Unicode::serialize(writer, text)?)
    }
}
