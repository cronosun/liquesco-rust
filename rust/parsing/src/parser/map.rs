
use liquesco_schema::any_type::AnyType;
use liquesco_schema::map::TMap;
use liquesco_schema::map::Sorting;
use crate::parser::value::TextValue;
use crate::parser::converter::Converter;
use crate::parser::core::Context;
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
        parse_map(context, writer, value, r#type.key(), r#type.value(), r#type.sorting(), r#type.length(), r#type.anchors(), true)
    }
}
