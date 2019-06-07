// It's also possible to have multiple levels of references.

mod common;

use common::builder::builder;
use common::builder::into_schema;
use common::utils::assert_invalid_strict;
use common::utils::assert_valid_strict;
use liquesco_schema::core::Schema;
use liquesco_schema::seq::TSeq;
use std::convert::TryFrom;

use liquesco_schema::identifier::Identifier;
use liquesco_schema::key_ref::TKeyRef;
use liquesco_schema::map::TMap;
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::structure::Field;
use liquesco_schema::structure::TStruct;
use liquesco_schema::unicode::LengthType;
use liquesco_schema::unicode::TUnicode;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[test]
fn ok_empty() {
    let schema = create_schema1();
    let map: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
    assert_valid_strict(map, &schema);
}

#[test]
fn ok1() {
    let schema = create_schema1();

    let mut inner_map1 = BTreeMap::new();
    inner_map1.insert("entry1".to_string(), Value{
        refs0: vec![],
        refs1: vec![0, 1]
    });
    inner_map1.insert("entry2".to_string(), Value{
        refs0: vec![0, 1, 2],
        refs1: vec![]
    });
    inner_map1.insert("entry3".to_string(), Value{
        refs0: vec![2, 1, 0],
        refs1: vec![0, 1]
    });

    let mut inner_map2 = BTreeMap::new();
    inner_map2.insert("entry1".to_string(), Value{
        refs0: vec![],
        refs1: vec![]
    });
    inner_map2.insert("entry2".to_string(), Value{
        refs0: vec![],
        refs1: vec![1, 0]
    });
    inner_map2.insert("entry3".to_string(), Value{
        refs0: vec![0, 1, 2, 3],
        refs1: vec![]
    });
    inner_map2.insert("entry4".to_string(), Value{
        refs0: vec![3, 1, 2, 0],
        refs1: vec![]
    });

    let mut outer_map = BTreeMap::new();
    outer_map.insert("outer1".to_string(), inner_map1);
    outer_map.insert("outer2".to_string(), inner_map2);

    assert_valid_strict(outer_map, &schema);
}

#[test]
fn err_outer_map_out_of_bounds() {
    let schema = create_schema1();

    let mut inner_map1 = BTreeMap::new();
    inner_map1.insert("entry1".to_string(), Value{
        refs0: vec![],
        refs1: vec![0, 1]
    });
    inner_map1.insert("entry2".to_string(), Value{
        refs0: vec![0, 1, 2],
        refs1: vec![]
    });
    inner_map1.insert("entry3".to_string(), Value{
        refs0: vec![2, 1, 0],
        refs1: vec![0, 1]
    });

    let mut inner_map2 = BTreeMap::new();
    inner_map2.insert("entry1".to_string(), Value{
        refs0: vec![],
        refs1: vec![]
    });
    inner_map2.insert("entry2".to_string(), Value{
        refs0: vec![],
        refs1: vec![1, 0]
    });
    inner_map2.insert("entry3".to_string(), Value{
        refs0: vec![0, 1, 2, 3],
        // NOTE: Here we reference the outer map (has only 2 entries): Index "2" is out of bounds
        refs1: vec![0, 1, 2]
    });
    inner_map2.insert("entry4".to_string(), Value{
        refs0: vec![3, 1, 2, 0],
        refs1: vec![]
    });

    let mut outer_map = BTreeMap::new();
    outer_map.insert("outer1".to_string(), inner_map1);
    outer_map.insert("outer2".to_string(), inner_map2);

    assert_invalid_strict(outer_map, &schema);
}

#[test]
fn err_inner_map_out_of_bounds() {
    let schema = create_schema1();

    let mut inner_map1 = BTreeMap::new();
    inner_map1.insert("entry1".to_string(), Value{
        refs0: vec![],
        refs1: vec![0, 1]
    });
    inner_map1.insert("entry2".to_string(), Value{
        refs0: vec![0, 1, 2],
        refs1: vec![]
    });
    inner_map1.insert("entry3".to_string(), Value{
        // Note: We reference index "3": That's out of bounds, inner map has only 3 entries.
        refs0: vec![2, 1, 0, 3],
        refs1: vec![0, 1]
    });

    let mut inner_map2 = BTreeMap::new();
    inner_map2.insert("entry1".to_string(), Value{
        refs0: vec![],
        refs1: vec![]
    });
    inner_map2.insert("entry2".to_string(), Value{
        refs0: vec![],
        refs1: vec![1, 0]
    });
    inner_map2.insert("entry3".to_string(), Value{
        refs0: vec![0, 1, 2, 3],
        refs1: vec![0, 1]
    });
    inner_map2.insert("entry4".to_string(), Value{
        refs0: vec![3, 1, 2, 0],
        refs1: vec![]
    });

    let mut outer_map = BTreeMap::new();
    outer_map.insert("outer1".to_string(), inner_map1);
    outer_map.insert("outer2".to_string(), inner_map2);

    assert_invalid_strict(outer_map, &schema);
}

fn create_schema1() -> impl Schema {
    let mut builder = builder();
    let single_ref_level0 = builder.add_unwrap("ref_level0", TKeyRef::default());
    let refs_level0 = builder.add_unwrap("refs_level0", TSeq::try_new(single_ref_level0, 0, 100).unwrap());
    let single_ref_level1 = builder.add_unwrap("ref_level1", TKeyRef::default().with_level(1));
    let refs_level1 = builder.add_unwrap("refs_level1", TSeq::try_new(single_ref_level1, 0, 100).unwrap());

    let value = builder.add_unwrap(
        "struct",
        TStruct::default()
            .add(Field::new(
                Identifier::try_from("refs0").unwrap(),
                refs_level0,
            ))
            .add(Field::new(
                Identifier::try_from("refs1").unwrap(),
                refs_level1,
            )),
    );

    let key = builder.add_unwrap(
        "key",
        TUnicode::try_new(0, 100, LengthType::Utf8Byte).unwrap(),
    );

    // Inner map
    let inner_map = builder.add_unwrap("inner_map", TMap::new(key.clone(), value).with_anchors(true));

    // outer map
    let outer_map = builder.add_unwrap("outer_map", TMap::new(key, inner_map).with_anchors(true));

    into_schema(builder, outer_map)
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Value {
    refs0: Vec<u32>,
    refs1: Vec<u32>,
}
