use crate::serde::assert_serde;
use crate::serde::serialize_to_same;
use serde::{Deserialize, Serialize};

#[test]
fn test_enum1() {
    assert_serde(Enum1::One);
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum Enum1 {
    One,
}

#[test]
fn test_enum2() {
    assert_serde(Enum2::One);
    assert_serde(Enum2::Two);
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum Enum2 {
    One,
    Two,
}

#[test]
fn test_enum3() {
    assert_serde(Enum3::One);
    assert_serde(Enum3::Two);
    assert_serde(Enum3::Three);
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum Enum3 {
    One,
    Two,
    Three,
}

#[test]
fn enum_with_data() {
    assert_serde(EnumWithData1::Item1);
    assert_serde(EnumWithData1::Item0("hello".to_string()));
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum EnumWithData1 {
    Item0(String),
    Item1,
}

#[test]
fn enum_with_data2() {
    assert_serde(EnumWithData2::Item0("".to_string(), 8, false));
    assert_serde(EnumWithData2::Item1);
    assert_serde(EnumWithData2::Item2(true));
    assert_serde(EnumWithData2::Item2(false));
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum EnumWithData2 {
    Item0(String, u8, bool),
    Item1,
    Item2(bool),
}

#[test]
fn enum_with_data3() {
    assert_serde(EnumWithData3::Item0 {
        name: "".to_string(),
        flag: 8,
        is_it_true: false,
    });
    assert_serde(EnumWithData3::Item1);
    assert_serde(EnumWithData3::Item2 { really: true });
    assert_serde(EnumWithData3::Item2 { really: false });
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum EnumWithData3 {
    Item0 {
        name: String,
        flag: u8,
        is_it_true: bool,
    },
    Item1,
    Item2 {
        really: bool,
    },
}

#[test]
fn struct_and_tuple_variants_encode_to_same() {
    serialize_to_same(
        EnumWithData3::Item0 {
            name: "my text".to_string(),
            flag: 78,
            is_it_true: false,
        },
        EnumWithData2::Item0("my text".to_string(), 78, false),
    );
    serialize_to_same(EnumWithData2::Item1, EnumWithData3::Item1);
    serialize_to_same(
        EnumWithData3::Item2 { really: true },
        EnumWithData2::Item2(true),
    );
}
