use crate::tests::serde::utils::assert_serde;
use crate::tests::serde::utils::serialize_to_same;
use maplit::btreemap;
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[test]
fn string_int_map() {
    let map = hashmap! {
        "a".to_string() => 1,
        "b".to_string() => 2,
        "some_key".to_string() => 2343
    };
    assert_serde(map);
}

/// A map is just a list of lists (or entries ... where each entry is tuple of key
/// and value).
#[test]
fn maps_are_just_list_in_lists() {
    let map1 = btreemap! {
    "a_my_key".to_string() => 587usize,
    "b_AnotherKey".to_string() => 2123usize,
    "c_and_this_is_the_last_key".to_string() => 4447usize
    };
    let map2: Vec<(String, usize)> = vec![
        ("a_my_key".to_string(), 587),
        ("b_AnotherKey".to_string(), 2123),
        ("c_and_this_is_the_last_key".to_string(), 4447),
    ];
    serialize_to_same(map1, map2);
}

#[test]
fn empty_map() {
    let map: HashMap<usize, usize> = HashMap::new();
    assert_serde(map);
}

#[test]
fn complex_key() {
    let map = hashmap! {
        MapKey { lo: 2212, hi : 212} => 54454,
        MapKey { lo: 44, hi : 444447} => 4,
        MapKey { lo: 144558, hi : 0} => -2
    };
    assert_serde(map);
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
struct MapKey {
    lo: usize,
    hi: usize,
}
