use liquesco_core::schema::anchors::TAnchors;

use liquesco_core::serialization::option::Presence;
use liquesco_core::serialization::core::Serializer;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::converter::Converter;

pub struct PAnchors;

impl Parser<'static> for PAnchors {
    type T = TAnchors;

    fn parse<'c, C>(context: &C, writer : &mut C::TWriter, r#type: &Self::T) -> Result<(), ParseError>
        where
            C: Context<'c> {
        C::TConverter::require_no_name(context.text_value())?;

        unimplemented!()
    }
}