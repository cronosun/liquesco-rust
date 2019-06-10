use crate::parser::core::Context;
use crate::parser::core::Parser;
use crate::parser::map_common::parse_map;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::types::map::TMap;

pub struct PMap;

impl<'a> Parser<'a> for PMap {
    type T = TMap<'a>;

    fn parse<'c, C>(
        context: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        r#type: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        parse_map(
            context,
            writer,
            value,
            r#type.key(),
            r#type.value(),
            r#type.sorting(),
            r#type.length(),
            r#type.anchors(),
            true,
        )
    }
}
