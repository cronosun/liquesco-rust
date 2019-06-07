
use liquesco_schema::any_type::AnyType;
use liquesco_schema::map::TMap;
use liquesco_schema::map::Sorting;
use crate::parser::value::TextValue;
use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::AnchorInfo;
use crate::parser::core::Parser;
use crate::parser::value::Seq;
use crate::parser::map_common::parse_map;
use liquesco_common::error::LqError;
use liquesco_schema::core::TypeRef;
use liquesco_schema::core::TypeContainer;
use liquesco_schema::core::Schema;
use std::io::Write;

use liquesco_serialization::core::Serializer;
use liquesco_serialization::seq::SeqHeader;
use liquesco_serialization::slice_reader::SliceReader;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::cmp::Ordering;
use liquesco_schema::root_map::TRootMap;
use liquesco_schema::key_ref::TKeyRef;
use liquesco_serialization::uint::UInt32;

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
        // first step is to read the key. Note: We have to clone the type (since cannot borrow
        // more than once, since parse needs a mutable context)
        let key_type = require_anchors(context, r#type.level())?.key_type().clone();

        let key_as_vec = context.parse_to_vec(&key_type, value)?;

        // now find the index
        let anchors = require_anchors(context, r#type.level())?.anchors();
        let maybe_index = anchors.get(&key_as_vec);
        if let Some(index) = maybe_index {
            UInt32::serialize(writer, index)
        } else {
            LqError::err_new(
                format!("You're trying to reference a key from an outer map (level {}). \
                    That key was not found in the outer map (note: keys in maps cannot reference \
                    itself). Given key value is {:?}.",
                        r#type.level(), value))
        }
    }
}

fn require_anchors<'c, C>(context : &C, level : u32) -> Result<&AnchorInfo, LqError>     where
    C: Context<'c>, {
    let anchors = context.anchors(level);
    if let Some(anchors) = anchors {
        Ok(anchors)
    } else {
        LqError::err_new(
            format!("There's no outer map (level {}) I could reference. Key refs can \
                only reference keys from outer maps - but there's none at given level. Note: \
                Keys in maps do not reference keys in the same map.",
                    level))
    }

}