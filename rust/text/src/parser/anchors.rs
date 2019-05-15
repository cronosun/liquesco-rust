use crate::parser::value::TextValue;
use liquesco_core::schema::anchors::TAnchors;

use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use liquesco_core::serialization::core::Serializer;
use liquesco_core::serialization::option::Presence;

pub struct PAnchors;

impl Parser<'static> for PAnchors {
    type T = TAnchors;

    fn parse<'c, C>(
        context: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        r#type: &Self::T,
    ) -> Result<(), ParseError>
    where
        C: Context<'c>,
    {
        C::TConverter::require_no_name(value)?;

        unimplemented!()
    }
}
