mod common;

use common::builder::builder;
use common::utils::assert_invalid_strict;
use common::utils::assert_valid_strict;
use liquesco_schema::core::Schema;
use liquesco_schema::seq::TSeq;
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use liquesco_schema::map::TMap;
use liquesco_schema::unicode::TUnicode;
use liquesco_schema::unicode::LengthType;
use std::collections::BTreeMap;
use liquesco_schema::key_ref::TKeyRef;
use liquesco_schema::structure::TStruct;
use liquesco_schema::structure::Field;
use liquesco_schema::identifier::Identifier;
use liquesco_schema::root_map::TRootMap;

#[test]
fn ok_empty() {
    let schema = create_schema1();
    let map : BTreeMap<String, Value> = BTreeMap::new();
    assert_valid_strict((map, RootValue{
        refs: vec![]
    }), &schema);
}

#[test]
fn ok_no_references() {
    let schema = create_schema1();
    let mut map : BTreeMap<String, Value> = BTreeMap::new();
    map.insert("item_a".to_string(), Value {
        text : "Some text".to_string(),
        refs : vec![],
    });
    map.insert("item_b".to_string(), Value {
        text : "Some other text".to_string(),
        refs : vec![],
    });
    assert_valid_strict((map, RootValue{ refs: vec![] }), &schema);
}

#[test]
fn ok_with_references() {
    let schema = create_schema1();
    let mut map : BTreeMap<String, Value> = BTreeMap::new();
    map.insert("item_a".to_string(), Value {
        text : "Some text".to_string(),
        refs : vec![0, 0, 0, 2],
    });
    map.insert("item_b".to_string(), Value {
        text : "Some other text".to_string(),
        refs : vec![2, 2, 2 ],
    });
    map.insert("item_c".to_string(), Value {
        text : "Some other text".to_string(),
        refs : vec![1],
    });
    // root references everything
    assert_valid_strict((map, RootValue{ refs: vec![0, 2, 1] }), &schema);
}

#[test]
fn err_out_of_index() {
    let schema = create_schema1();
    let mut map : BTreeMap<String, Value> = BTreeMap::new();
    map.insert("item_a".to_string(), Value {
        text : "Some text".to_string(),
        refs : vec![0, 0, 0, 2],
    });
    map.insert("item_b".to_string(), Value {
        text : "Some other text".to_string(),
        refs : vec![2, 2, 2],
    });
    map.insert("item_c".to_string(), Value {
        text : "Some other text".to_string(),
        refs : vec![1],
    });
    // note: "3" is out of bounds
    assert_invalid_strict((map, RootValue{ refs: vec![3] }), &schema);
}

fn create_schema1() -> impl Schema<'static> {
    let mut builder = builder();
    let key = builder.add(TUnicode::try_new(0, 100, LengthType::Utf8Byte).unwrap());

    let field_text = builder.add(TUnicode::try_new(0, 100, LengthType::Utf8Byte).unwrap());
    let single_ref = builder.add(TKeyRef::default());
    let field_refs = builder.add(TSeq::try_new(single_ref, 0, 100).unwrap());
    let value = builder.add(TStruct::default().add(
        Field::new(Identifier::try_from("text").unwrap(), field_text)
    ).add(
        Field::new(Identifier::try_from("refs").unwrap(), field_refs)
    ));

    let root = builder.add(TStruct::default().add(
        Field::new(Identifier::try_from("refs").unwrap(), field_refs)
    ));

    builder.finish(TRootMap::new(root, key, value))
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Value {
    text : String,
    refs : Vec<u32>
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct RootValue {
    refs : Vec<u32>
}