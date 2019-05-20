use crate::tests::serde::utils::can_decode_from;
use serde::{Deserialize, Serialize};

/// It's always OK when there's more input data than required (this
/// can be used for schema evolution).
///
/// Source has more fields than destination.
#[test]
fn structs_are_extensible() {
    let source = Source1 {
        prefix: 45875,
        content: Source1Inner {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            age: 45,
            student: true,
        },
        postfix: 7777777,
        another_field: 4534534,
    };

    // that's what we expect to get (all other fields should be ignored)
    let destination = Destination1 {
        prefix: 45875,
        content: Destination1Inner {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        },
        postfix: 7777777,
    };

    can_decode_from(source, destination);
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Source1Inner {
    first_name: String,
    last_name: String,
    age: u32,
    student: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Source1 {
    prefix: usize,
    content: Source1Inner,
    postfix: usize,
    another_field: usize,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Destination1Inner {
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Destination1 {
    prefix: usize,
    content: Destination1Inner,
    postfix: usize,
}

#[test]
fn tuples_are_extensible() {
    let source = (
        (458, 477, true, "hello".to_string(), "extension".to_string()),
        (
            "world".to_string(),
            false,
            ("bonjour".to_string(), true, 48),
            34,
            "inner tuple end".to_string(),
        ),
        -478,
        false,
        (),
        (true, true, false, (false, false, true)),
        "end_here".to_string(),
    );

    let destination = (
        (458, 477, true, "hello".to_string()),
        ("world".to_string(), false, ("bonjour".to_string(), true)),
        -478,
        false,
        (),
    );

    can_decode_from(source, destination);
}
