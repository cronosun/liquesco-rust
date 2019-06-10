use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::Parser;
use crate::parser::map_common::parse_map;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;

use liquesco_serialization::core::Serializer;
use liquesco_serialization::types::seq::SeqHeader;

use liquesco_schema::root_map::TRootMap;

pub struct PRootMap;

impl<'a> Parser<'a> for PRootMap {
    type T = TRootMap<'a>;

    fn parse<'c, C>(
        context: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        r#type: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        // we need [root, [map]]
        let outer_seq = C::TConverter::require_seq(&value.value)?;
        if outer_seq.len() != 2 {
            return LqError::err_new(format!(
                "The root map looks like this: \
                 [root, [[key1, entry2], [key2, entry2], ...]]; So the outer sequence has to have \
                 2 entries: The root and the map. Your sequence has {} entries. Value: {:?}.",
                outer_seq.len(),
                value
            ));
        }

        SeqHeader::serialize(writer, &SeqHeader::new(2))?;
        // First parse the map. Note: We do not pop the anchors (so the root serialization has the anchors)
        parse_map(
            context,
            writer,
            &outer_seq[1],
            r#type.key(),
            r#type.value(),
            r#type.sorting(),
            r#type.length(),
            true,
            false,
        )?;

        context.parse(writer, r#type.root(), &outer_seq[0])?;

        // Here pop the anchors
        context.pop_anchors()?;
        Ok(())
    }
}
