mod common;

use common::builder::builder;
use common::utils::assert_invalid_strict;
use common::utils::assert_valid_strict;
use liquesco_schema::core::Schema;

use crate::common::builder::into_schema;
use liquesco_schema::map::TMap;
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::sint::TSInt;
use liquesco_schema::unicode::LengthType;
use liquesco_schema::unicode::TUnicode;
use std::collections::BTreeMap;

#[test]
fn ok_empty_map() {
    let schema = create_schema1();
    let map: Vec<(String, String)> = vec![];
    assert_valid_strict(map, &schema);
}

#[test]
fn ok_map() {
    let schema = create_schema1();
    let map: Vec<(String, String)> = vec![
        ("aaa".to_string(), "C Some Value".to_string()),
        ("bbb".to_string(), "A Some Value".to_string()),
        ("ccc".to_string(), "B Some Value".to_string()),
    ];
    assert_valid_strict(map, &schema);
}

#[test]
fn ok_map_numbers() {
    let schema = create_schema_numbers();
    let map: Vec<(isize, isize)> = vec![(10, 30), (20, 20), (30, 10), (70, 50), (71, 50)];
    assert_valid_strict(map, &schema);
}

#[test]
fn err_wrong_ordering_string() {
    let schema = create_schema1();
    let map: Vec<(String, String)> = vec![
        ("bbb".to_string(), "C Some Value".to_string()),
        ("ccc".to_string(), "A Some Value".to_string()),
        ("aaa".to_string(), "B Some Value".to_string()),
    ];
    assert_invalid_strict(map, &schema);
}

#[test]
fn err_wrong_key_ordering() {
    let schema = create_schema_numbers();
    let map: Vec<(isize, isize)> = vec![(10, 30), (11, 30), (9, 20)];
    assert_invalid_strict(map, &schema);
}

/// When using BTreeMap we get correct key ordering automatically
#[test]
fn ok_with_btree() {
    let mut my_map: BTreeMap<isize, isize> = BTreeMap::new();
    for index in 0..300 {
        let key = index % 31;
        let value = index % 7;
        my_map.insert(key, value);
    }

    let schema = create_schema_numbers();
    assert_valid_strict(my_map, &schema);
}

fn create_schema1() -> impl Schema {
    let mut builder = builder();
    let key = builder.add_unwrap(
        "key",
        TUnicode::try_new(0, 100, LengthType::Utf8Byte).unwrap(),
    );
    let value = builder.add_unwrap(
        "value",
        TUnicode::try_new(0, 100, LengthType::Utf8Byte).unwrap(),
    );

    into_schema(builder, TMap::new(key, value))
}

fn create_schema_numbers() -> impl Schema {
    let mut builder = builder();
    let key = builder.add_unwrap("key", TSInt::try_new(-1000, 1000).unwrap());
    let value = builder.add_unwrap("value", TSInt::try_new(-1000, 1000).unwrap());

    into_schema(builder, TMap::new(key, value))
}
