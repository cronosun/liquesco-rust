use crate::serde::assert_serde;
use serde::{Deserialize, Serialize};

#[test]
fn string() {
    assert_serde(String::from("hello world!"));
    assert_serde(String::from(""));
}

#[test]
fn uints() {
    assert_serde(8u8);
    assert_serde(27454u16);
    assert_serde(1_227_454u32);
    assert_serde(std::u64::MAX);
    assert_serde(std::u64::MIN);
}

#[test]
fn sints() {
    assert_serde(-8i8);
    assert_serde(-27454i16);
    assert_serde(-1_227_454i32);
    assert_serde(std::i64::MIN);
    assert_serde(std::i64::MAX);
}

#[test]
fn floats() {
    assert_serde(1.8f32);
    assert_serde(1.8f64);
}

#[test]
fn boolean() {
    assert_serde(true);
    assert_serde(false);
}

#[test]
fn character() {
    assert_serde('a');
    assert_serde('!');
}

#[test]
fn option() {
    let none: Option<String> = Option::None;
    let some = Option::Some("this is a string".to_string());
    assert_serde(none);
    assert_serde(some);
}

#[test]
fn unit() {
    assert_serde(());
}

#[test]
fn unit_struct() {
    let unit_struct = UnitStruct;
    assert_serde(unit_struct);
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct UnitStruct;

#[test]
fn str_newtype() {
    let newtype = Newtype("Some string".to_string());
    assert_serde(newtype);
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Newtype(String);

#[test]
fn int_newtype() {
    let newtype = IntNewtype(45875);
    assert_serde(newtype);
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct IntNewtype(usize);
