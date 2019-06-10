use crate::parser::core::AnchorInfo;
use crate::parser::core::Context;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;

use liquesco_serialization::core::Serializer;

use liquesco_schema::key_ref::TKeyRef;
use liquesco_serialization::types::uint::UInt32;

pub struct PKeyRef;

impl<'a> Parser<'a> for PKeyRef {
    type T = TKeyRef<'a>;

    fn parse<'c, C>(
        context: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        r#type: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let anchor_info = require_anchors(context, r#type.level())?;

        let key_as_vec = context.parse_to_vec(anchor_info.key_type(), value)?;

        // now find the index
        let anchors = anchor_info.anchors();
        let maybe_index = anchors.get(&key_as_vec);
        if let Some(index) = maybe_index {
            UInt32::serialize(writer, index)
        } else {
            LqError::err_new(format!(
                "You're trying to reference a key from an outer map (level {}). \
                 That key was not found in the outer map (note: keys in maps cannot reference \
                 itself). Given key value is {:?}.",
                r#type.level(),
                value
            ))
        }
    }
}

fn require_anchors<'c, C>(context: &C, level: u32) -> Result<&AnchorInfo, LqError>
where
    C: Context<'c>,
{
    let anchors = context.anchors(level);
    if let Some(anchors) = anchors {
        Ok(anchors)
    } else {
        LqError::err_new(format!(
            "There's no outer map (level {}) I could reference. Key refs can \
             only reference keys from outer maps - but there's none at given level. Note: \
             Keys in maps do not reference keys in the same map.",
            level
        ))
    }
}
