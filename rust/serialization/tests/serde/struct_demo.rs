use serde::{Deserialize, Serialize};
use std::string::ToString;
use crate::serde::assert_serde;

#[test]
fn struct_demo() {
    let source = Main {
        integer: 54,
        embedded1: Embedded1{},
        embedded2: Embedded2 {
            first_name : "John".to_string(),
            last_name : "Doe".to_string(),
        },
    };
    assert_serde(source);
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Main {
    integer: u8,
    embedded1 : Embedded1,
    embedded2 : Embedded2
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Embedded1 {
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Embedded2 {
    first_name: String,
    last_name: String,
}

