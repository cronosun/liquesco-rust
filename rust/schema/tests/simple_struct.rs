mod common;

use common::builder::builder;
use common::builder::into_schema;
use common::ordering::ord_schema;
use common::utils::assert_invalid_extended;
use common::utils::assert_invalid_strict;
use common::utils::assert_valid_extended;
use common::utils::assert_valid_strict;
use common::utils::id;
use liquesco_schema::types::ascii::TAscii;
use liquesco_schema::types::boolean::TBool;
use liquesco_schema::core::Schema;
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::types::seq::Direction;
use liquesco_schema::types::sint::TSInt;
use liquesco_schema::types::structure::Field;
use liquesco_schema::types::structure::TStruct;
use liquesco_schema::types::uint::TUInt;
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[test]
fn schema1() {
    let mut builder = builder();
    let int = builder.add_unwrap("int", TUInt::try_new(2, 144).unwrap());
    let upper_case = builder.add_unwrap("upper_case", TAscii::try_new(2, 10, 65, 90).unwrap());
    let root = builder.add_unwrap(
        "root",
        TStruct::default()
            .add(Field::new(id("age"), int))
            .add(Field::new(id("name"), upper_case)),
    );
    let schema = into_schema(builder, root);

    // valid
    assert_valid_strict(
        Schema1Struct {
            age: 45,
            name: "JOHN".to_string(),
        },
        &schema,
    );
    assert_valid_strict(
        Schema1Struct {
            age: 144,
            name: "ZZZZZZZZZZ".to_string(),
        },
        &schema,
    );
    assert_valid_strict(
        Schema1Struct {
            age: 2,
            name: "AA".to_string(),
        },
        &schema,
    );

    // invalid
    assert_invalid_strict(
        Schema1Struct {
            age: 1,
            name: "AA".to_string(),
        },
        &schema,
    );
    assert_invalid_strict(
        Schema1Struct {
            age: 20,
            name: "aaa".to_string(),
        },
        &schema,
    );
    assert_invalid_strict(Schema1StructShort { age: 20 }, &schema);
    assert_invalid_strict(
        Schema1StructLong {
            age: 20,
            name: "JOHN".to_string(),
            alive: true,
        },
        &schema,
    );
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Schema1Struct {
    age: u64,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Schema1StructShort {
    age: u64,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Schema1StructLong {
    age: u64,
    name: String,
    alive: bool,
}

fn ordering_create_schema() -> impl Schema {
    ord_schema(
        |builder| {
            let type_x = builder.add_unwrap("type_x", TUInt::try_new(0, std::u64::MAX).unwrap());
            let type_y = builder.add_unwrap(
                "type_y",
                TSInt::try_new(std::i64::MIN, std::i64::MAX).unwrap(),
            );
            let inner_struct = builder.add_unwrap(
                "inner_struct",
                Into::<TStruct>::into(
                    TStruct::default()
                        .add(Field::new(id("x"), type_x))
                        .add(Field::new(id("y"), type_y)),
                ),
            );
            let type_more = builder.add_unwrap("type_more", TBool::default());
            builder.add_unwrap(
                "struct",
                Into::<TStruct>::into(
                    TStruct::default()
                        .add(Field::new(id("content"), inner_struct))
                        .add(Field::new(id("more"), type_more)),
                ),
            )
        },
        Direction::Ascending,
        true,
    )
}

#[test]
fn ordering_no_extension() {
    let schema = ordering_create_schema();

    // "normal" check (no extensions) - valid
    assert_valid_strict(
        (
            ContainerForOrdering1 {
                content: StructForOrdering1 { x: 10, y: 20 },
                more: false,
            },
            ContainerForOrdering1 {
                content: StructForOrdering1 { x: 10, y: 20 },
                more: true,
            },
            ContainerForOrdering1 {
                content: StructForOrdering1 { x: 10, y: 21 },
                more: false,
            },
        ),
        &schema,
    );

    // "normal" check (no extensions) - invalid because of duplicate
    assert_invalid_strict(
        (
            ContainerForOrdering1 {
                content: StructForOrdering1 { x: 10, y: 20 },
                more: false,
            },
            ContainerForOrdering1 {
                content: StructForOrdering1 { x: 10, y: 20 },
                more: false,
            },
        ),
        &schema,
    );

    // "normal" check (no extensions) - invalid because wrong ordering
    assert_invalid_strict(
        (
            ContainerForOrdering1 {
                content: StructForOrdering1 { x: 10, y: 20 },
                more: true,
            },
            ContainerForOrdering1 {
                content: StructForOrdering1 { x: 10, y: 20 },
                more: false,
            },
        ),
        &schema,
    );
}

/// Here we test that only the parts defined in the schema are used to compare
/// two items.
#[test]
fn ordering_extension() {
    let schema = ordering_create_schema();

    assert_valid_extended(
        (
            ContainerForOrdering1 {
                content: StructForOrdering1 { x: 10, y: 20 },
                more: false,
            },
            ContainerForOrdering1Ext {
                content: StructForOrdering1Ext {
                    x: 10,
                    y: 20,
                    more_data: "This field is ignored".to_string(),
                },
                more: true,
                more2: false,
            },
        ),
        &schema,
    );

    // See: They're not byte-equal but are considered to be equal (since
    // 'more_data' is not in the schema, so we don't use that for comparison)
    assert_invalid_extended(
        (
            ContainerForOrdering1 {
                content: StructForOrdering1 { x: 10, y: 20 },
                more: false,
            },
            ContainerForOrdering1Ext {
                content: StructForOrdering1Ext {
                    x: 10,
                    y: 20,
                    more_data: "This field is ignored".to_string(),
                },
                more: false,
                more2: false,
            },
        ),
        &schema,
    );
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct ContainerForOrdering1 {
    content: StructForOrdering1,
    more: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct StructForOrdering1 {
    x: usize,
    y: isize,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct ContainerForOrdering1Ext {
    content: StructForOrdering1Ext,
    more: bool,
    more2: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct StructForOrdering1Ext {
    x: usize,
    y: isize,
    more_data: String,
}
