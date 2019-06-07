// Keys cannot reference itself

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
    let map: BTreeMap<String, BTreeMap<Key, Value>> = BTreeMap::new();
    assert_valid_strict(map, &schema);
}

#[test]
fn ok1() {
    let schema = create_schema1();

    let mut inner_map1 = BTreeMap::new();
    inner_map1.insert(
        Key(
            // this is the important part: Level is 0 but this references the OUTER map
            vec![],
        ),
        Value {
            refs0: vec![],
            refs1: vec![0, 1],
        },
    );
    inner_map1.insert(
        Key(
            // this is the important part: Level is 0 but this references the OUTER map
            vec![0],
        ),
        Value {
            refs0: vec![0, 1, 2],
            refs1: vec![],
        },
    );
    inner_map1.insert(
        Key(
            // this is the important part: Level is 0 but this references the OUTER map
            vec![1, 0],
        ),
        Value {
            refs0: vec![2, 1, 0],
            refs1: vec![0, 1],
        },
    );

    let mut inner_map2 = BTreeMap::new();
    inner_map2.insert(
        Key(
            // this is the important part: Level is 0 but this references the OUTER map
            vec![1, 0],
        ),
        Value {
            refs0: vec![],
            refs1: vec![],
        },
    );

    let mut outer_map = BTreeMap::new();
    outer_map.insert("outer1".to_string(), inner_map1);
    outer_map.insert("outer2".to_string(), inner_map2);

    assert_valid_strict(outer_map, &schema);
}

#[test]
fn err_ref_out_of_bounds() {
    let schema = create_schema1();

    let mut inner_map1 = BTreeMap::new();
    inner_map1.insert(
        Key(vec![]),
        Value {
            refs0: vec![],
            refs1: vec![0, 1],
        },
    );
    inner_map1.insert(
        Key(vec![0]),
        Value {
            refs0: vec![0, 1, 2],
            refs1: vec![],
        },
    );
    inner_map1.insert(
        Key(
            // IMPORTANT PART: If this would reference the "inner" map, the "2" would be ok, but it references the outer map: So it's out of bounds.
            vec![1, 0, 2],
        ),
        Value {
            refs0: vec![2, 1, 0],
            refs1: vec![0, 1],
        },
    );

    let mut inner_map2 = BTreeMap::new();
    inner_map2.insert(
        Key(vec![1, 0]),
        Value {
            refs0: vec![],
            refs1: vec![],
        },
    );

    let mut outer_map = BTreeMap::new();
    outer_map.insert("outer1".to_string(), inner_map1);
    outer_map.insert("outer2".to_string(), inner_map2);

    assert_invalid_strict(outer_map, &schema);
}

fn create_schema1() -> impl Schema {
    let mut builder = builder();
    let single_ref_level0 = builder.add_unwrap("ref_level0", TKeyRef::default());
    let refs_level0 = builder.add_unwrap(
        "refs_level0",
        TSeq::try_new(single_ref_level0, 0, 100).unwrap(),
    );
    let single_ref_level1 = builder.add_unwrap("ref_level1", TKeyRef::default().with_level(1));
    let refs_level1 = builder.add_unwrap(
        "refs_level1",
        TSeq::try_new(single_ref_level1, 0, 100).unwrap(),
    );

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

    let key_entry = builder.add_unwrap("key_entry", TKeyRef::default());
    let key_inner = builder.add_unwrap("key_inner", TSeq::try_new(key_entry, 0, 100).unwrap());
    let key_outer = builder.add_unwrap(
        "key_outer",
        TUnicode::try_new(0, 100, LengthType::Utf8Byte).unwrap(),
    );

    // Inner map
    let inner_map = builder.add_unwrap("inner_map", TMap::new(key_inner, value).with_anchors(true));

    // outer map
    let outer_map = builder.add_unwrap(
        "outer_map",
        TMap::new(key_outer, inner_map).with_anchors(true),
    );

    into_schema(builder, outer_map)
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Value {
    refs0: Vec<u32>,
    refs1: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Key(Vec<u32>);
