
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
        parse_map(context, writer, value, r#type.key(), r#type.value(), r#type.sorting(), r#type.length(), r#type.anchors())

        // must be a sequence: [[key1, value1], [key2, value2], ...]
        /*let outer_seq = C::TConverter::require_seq(&value.value)?;
        let number_of_entries = outer_seq.len();
        let u32_number_of_entries = u32::try_from(number_of_entries)?;*/

        // Need to do some more when this provides anchors
        // TODO -> This does not really work, since keys have not yet been sorted
        /*let saved_anchors = if r#type.anchors() {
            let new_anchors = read_all_keys_to_vec(context, outer_seq, r#type.key())?;
            let removed_anchors = context.remove_anchors();
            context.set_anchors(Some(new_anchors));
            removed_anchors
        } else {
            None
        };*/

        // TODO: We do not really need to process values here...
        //let map_as_vec = parse_map_to_vec(context, outer_seq, r#type.key(), r#type.value())?;
        // Now we have to sort according to keys
        //let sorted_map_as_vec = sort_map_as_vec(context, outer_seq, r#type.key(), r#type.value(), map_as_vec, r#type.sorting())?;

        // now write everything
        /*SeqHeader::serialize(writer, &SeqHeader::new(u32_number_of_entries))?;
        for entry in sorted_map_as_vec {
            let (key, value) = entry;
            SeqHeader::serialize(writer, &SeqHeader::new(2))?;
            writer.write_all(key.as_slice())?;
            writer.write_all(value.as_slice())?;
        }*/

        // Maybe restore anchors
        /*if r#type.anchors() {
            context.set_anchors(saved_anchors);
        }*/

        //Ok(())
    }
}

/*
fn sort_map_as_vec<'c, C>(
    context: &mut C,
    outer_seq: &Seq,
    key_type: &TypeRef,
    value_type : &TypeRef,
    mut unsorted_vec : Vec<(Vec<u8>, Vec<u8>)>,
    sorting : Sorting,
) -> Result<Vec<(Vec<u8>, Vec<u8>)>, LqError> where
    C: Context<'c>, {
        //let key_type : &AnyType = context.schema().require_type(key_type)?;           
        let mut error = false;

        unsorted_vec.sort_by(|a, b| {
            let (a_key, _) = a;
            let (b_key, _) = b;

            let mut a_reader = SliceReader::from(a_key);
            let mut b_reader = SliceReader::from(b_key);

            let result = context.schema().compare(key_type, &mut a_reader, &mut b_reader);
            if let Ok(result) = result {
                result
            } else {
                error = true;
                Ordering::Equal
            }
        });

        if error {
            LqError::err_new("Unable to sort keys in the given map")
        } else {
            Ok(unsorted_vec)
        }
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
        let value_as_vec = context.parse_to_vec(value_type, &inner_seq[1])?;
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
             ...]. So the inner seq has to have exactly 2 elements; it has {} elements. Map \
             entry which should have 2 elements is {:?}.",
            inner_seq.len(),
            inner_seq
        ))
    } else {
        Ok(())
    }
}*/