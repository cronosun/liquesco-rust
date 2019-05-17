use crate::schema::doc_type::DocType;
use crate::schema::core::Schema;
use crate::schema::ascii::TAscii;
use crate::schema::boolean::TBool;
use crate::schema::seq::Direction;
use crate::schema::sint::TSInt;
use crate::schema::structure::TStruct;
use crate::schema::uint::TUInt;
use crate::tests::schema::builder::builder;
use crate::tests::schema::ordering::ord_schema;
use crate::tests::schema::utils::assert_invalid_extended;
use crate::tests::schema::utils::assert_valid_extended;
use crate::tests::schema::utils::id;

use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;
use serde::{Deserialize, Serialize};

#[test]
fn schema1() {
    let mut builder = builder();
    let int = builder.add(DocType::from(TUInt::try_new(2, 144).unwrap()));
    let upper_case = builder.add(DocType::from(TAscii::try_new(2, 10, 65, 90).unwrap()));
    let schema = builder.finish(
        DocType::from(TStruct::builder()
            .field(id("age"), int)
            .field(id("name"), upper_case)
            .build()),
    );

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

fn ordering_create_schema() -> impl Schema<'static> {
    ord_schema(
        |builder| {
            let type_x = builder.add(DocType::from(TUInt::try_new(0, std::u64::MAX).unwrap()));
            let type_y = builder.add(DocType::from(TSInt::try_new(std::i64::MIN, std::i64::MAX).unwrap()));
            let inner_struct = builder.add(
                Into::<DocType<TStruct>>::into(TStruct::builder()
                    .field(id("x"), type_x)
                    .field(id("y"), type_y)
                    .build())
            );
            let type_more = builder.add(DocType::from(TBool::default()));
            builder.add(
                Into::<DocType<TStruct>>::into(TStruct::builder()
                    .field(id("content"), inner_struct)
                    .field(id("more"), type_more)
                    .build()),
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
