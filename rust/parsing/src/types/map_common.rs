// Common utilities for map parsing

use crate::converter::Converter;
use crate::core::AnchorInfo;
use crate::core::Context;
use crate::value::Seq;
use crate::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::core::Schema;
use liquesco_schema::core::TypeRef;
use liquesco_schema::types::map::Sorting;
use std::io::Write;

use liquesco_serialization::core::Serializer;
use liquesco_serialization::slice_reader::SliceReader;
use liquesco_serialization::types::seq::SeqHeader;

use liquesco_common::ine_range::U32IneRange;
use liquesco_common::range::LqRangeBounds;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;

pub struct PMap;

pub(crate) fn parse_map<'c, C>(
    context: &mut C,
    writer: &mut C::TWriter,
    value: &TextValue,
    key_type: &TypeRef,
    value_type: &TypeRef,
    sorting: Sorting,
    length: &U32IneRange,
    anchors: bool,
    // you want this to be true (usually)
    pop_anchors: bool,
) -> Result<(), LqError>
where
    C: Context<'c>,
{
    let map_as_vec = process_map_init(context, value, key_type, length)?;
    let vec_sorted = sort(context, key_type, sorting, map_as_vec)?;
    let anchors_map = to_anchors_map(&vec_sorted);
    if anchors {
        let anchor_info = AnchorInfo::new(anchors_map, key_type.clone());
        context.push_anchors(anchor_info);
    }
    write_map(context, writer, value_type, &vec_sorted)?;
    if anchors && pop_anchors {
        context.pop_anchors()?;
    }
    Ok(())
}

fn to_anchors_map(vec: &Vec<(Vec<u8>, &TextValue)>) -> HashMap<Vec<u8>, u32> {
    let mut index: u32 = 0;
    let mut result = HashMap::with_capacity(vec.len());
    for entry in vec {
        result.insert(entry.0.clone(), index);
        index = index + 1;
    }
    result
}

fn write_map<'a, 'c, C>(
    context: &mut C,
    writer: &mut C::TWriter,
    value_type: &TypeRef,
    map: &Vec<(Vec<u8>, &'a TextValue<'a>)>,
) -> Result<(), LqError>
where
    C: Context<'c>,
{
    let u32_number_of_entries = u32::try_from(map.len())?;
    SeqHeader::serialize(writer, &SeqHeader::new(u32_number_of_entries))?;
    for entry in map {
        let (key, value) = entry;
        SeqHeader::serialize(writer, &SeqHeader::new(2))?;
        writer.write_all(key.as_slice())?;
        // parse & write value
        context.parse(writer, value_type, value)?;
    }
    Ok(())
}

fn process_map_init<'a, 'c, C>(
    context: &mut C,
    value: &'a TextValue<'a>,
    key_type: &TypeRef,
    length: &U32IneRange,
) -> Result<Vec<(Vec<u8>, &'a TextValue<'a>)>, LqError>
where
    C: Context<'c>,
{
    let outer_seq = C::TConverter::require_seq(&value.value)?;
    let number_of_entries = outer_seq.len();
    let u32_number_of_entries = u32::try_from(number_of_entries)?;

    // check length
    length.require_within(
        "Map validation (length; number of entries).",
        &u32_number_of_entries,
    )?;

    let mut result = Vec::with_capacity(number_of_entries);
    for value_element in outer_seq {
        let inner_seq = C::TConverter::require_seq(&value_element.value)?;
        assert_inner_seq_len(inner_seq)?;

        let key_as_vec = context.parse_to_vec(key_type, &inner_seq[0])?;
        result.push((key_as_vec, &inner_seq[1]));
    }
    Ok(result)
}

fn sort<'a, 'c, C>(
    context: &mut C,
    key_type: &TypeRef,
    sorting: Sorting,
    mut unsorted_vec: Vec<(Vec<u8>, &'a TextValue<'a>)>,
) -> Result<Vec<(Vec<u8>, &'a TextValue<'a>)>, LqError>
where
    C: Context<'c>,
{
    let mut error = false;

    unsorted_vec.sort_by(|a, b| {
        let (a_key, _) = a;
        let (b_key, _) = b;

        let mut a_reader = SliceReader::from(a_key);
        let mut b_reader = SliceReader::from(b_key);

        let result = context
            .schema()
            .compare(key_type, &mut a_reader, &mut b_reader);
        if let Ok(result) = result {
            match sorting {
                Sorting::Ascending => result,
                Sorting::Descending => result.reverse(),
            }
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
}
