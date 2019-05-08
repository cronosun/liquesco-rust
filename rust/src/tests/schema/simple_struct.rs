use crate::schema::vascii::VAscii;
use crate::schema::vstruct::VStruct;
use crate::schema::vuint::VUInt;
use crate::tests::schema::builder::builder;
use crate::tests::schema::utils::id;

use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;
use serde::{Deserialize, Serialize};

#[test]
fn schema1() {
    let mut builder = builder();
    let int = builder.add(VUInt::try_new(2, 144).unwrap());
    let upper_case = builder.add(VAscii::try_new(2, 10, 65, 90).unwrap());
    let schema = builder.finish(
        VStruct::builder()
            .field(id("age"), int)
            .field(id("name"), upper_case)
            .build(),
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
