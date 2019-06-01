use crate::serde::assert_serde;
use crate::serde::serialize_to_same;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[test]
fn empty_map() {
    let map : HashMap<String, bool> = HashMap::new();
    assert_serde(map);
}

#[test]
fn simple_map() {
    let mut map : HashMap<String, String> = HashMap::new();
    map.insert("entry1".to_string(), "value1".to_string());
    map.insert("entry2".to_string(), "value2".to_string());

    assert_serde(map);
}
