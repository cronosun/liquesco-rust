
use liquesco_schema::map::TMap;
use liquesco_schema::map::Sorting;
use crate::parser::value::TextValue;
use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::Parser;
use crate::parser::value::Seq;
use liquesco_common::error::LqError;
use liquesco_schema::core::TypeRef;
use liquesco_schema::core::TypeContainer;

use liquesco_serialization::core::Serializer;
use liquesco_serialization::seq::SeqHeader;

use std::collections::HashMap;
use std::convert::TryFrom;
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
        // must be a sequence: [[key1, value1], [key2, value2], ...]
        let outer_seq = C::TConverter::require_seq(&value.value)?;
        let number_of_entries = outer_seq.len();
        let u32_number_of_entries = u32::try_from(number_of_entries)?;

        // Need to do some more when this provides anchors
        let saved_anchors = if r#type.anchors() {
            let new_anchors = read_all_keys_to_vec(context, outer_seq, r#type.key())?;
            let removed_anchors = context.remove_anchors();
            context.set_anchors(Some(new_anchors));
            removed_anchors
        } else {
            None
        };

        let map_as_vec = parse_map_to_vec(context, outer_seq, r#type.key(), r#type.value())?;



    /// OLD
        SeqHeader::serialize(writer, &SeqHeader::new(u32_number_of_entries))?;
        for value_element in outer_seq {
            let inner_seq = C::TConverter::require_seq(&value_element.value)?;
            if inner_seq.len() != 2 {
                return LqError::err_new(format!(
                    "A map has to look like this [[key1, value1], \
                     ...]. So the inner seq has to have exactly 2 elements; it has {} elements.",
                    inner_seq.len()
                ));
            }
            SeqHeader::serialize(writer, &SeqHeader::new(2))?;
            context.parse(writer, r#type.key(), &inner_seq[0])?;
            context.parse(writer, r#type.value(), &inner_seq[1])?;
        }

        // Maybe restore anchors
        if r#type.anchors() {
            context.set_anchors(saved_anchors);
        }

        Ok(())
    }
}

fn sort_map_as_vec<'c, C>(
    context: &mut C,
    outer_seq: &Seq,
    key_type: &TypeRef,
    value_type : &TypeRef,
    mut unsorted_vec : Vec<(Vec<u8>, Vec<u8>)>,
    sorting : Sorting,
) -> Result<Vec<(Vec<u8>, Vec<u8>)>, LqError> where
    C: Context<'c>, {
        let key_type = context.schema().require_type(key_type)?;           

        unsorted_vec.sort_by(|a, b| {
            let (a_key, _) = a;
            let (b_key, _) = b;

           // context.schema().compare(key_type, a_key.into(), b_key.into());



            unimplemented!()
        });
        Ok(unsorted_vec)
    }

fn parse_map_to_vec<'c, C>(
    context: &mut C,
    outer_seq: &Seq,
    key_type: &TypeRef,
    value_type : &TypeRef,
) -> Result<Vec<(Vec<u8>, Vec<u8>)>, LqError> where
    C: Context<'c>, {
    
    let mut result = Vec::with_capacity(outer_seq.len());
    
    for value_element in outer_seq {
        let inner_seq = C::TConverter::require_seq(&value_element.value)?;
        assert_inner_seq_len(inner_seq)?;

        let key_as_vec = context.parse_to_vec(key_type, &inner_seq[0])?;
        let value_as_vec = context.parse_to_vec(key_type, &inner_seq[1])?;
        result.push((key_as_vec, value_as_vec));
    }

    Ok(result)
}

fn read_all_keys_to_vec<'c, C>(
    context: &mut C,
    outer_seq: &Seq,
    key_type: &TypeRef,
) -> Result<HashMap<Vec<u8>, u32>, LqError>
where
    C: Context<'c>,
{
    let mut saved_keys = HashMap::<Vec<u8>, u32>::new();
    let mut index: u32 = 0;
    for value_element in outer_seq {
        let inner_seq = C::TConverter::require_seq(&value_element.value)?;
        assert_inner_seq_len(inner_seq)?;

        let key_as_vec = context.parse_to_vec(key_type, &inner_seq[0])?;
        saved_keys.insert(key_as_vec, index);
        index += 1;
    }
    Ok(saved_keys)
}

fn assert_inner_seq_len(inner_seq: &Seq) -> Result<(), LqError> {
    if inner_seq.len() != 2 {
        LqError::err_new(format!(
            "A map has to look like this [[key1, value1], \
             ...]. So the inner seq has to have exactly 2 elements; it has {} elements.",
            inner_seq.len()
        ))
    } else {
        Ok(())
    }
}