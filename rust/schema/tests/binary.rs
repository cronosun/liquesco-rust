mod common;

use common::builder::builder;
use common::utils::assert_invalid_strict;
use common::utils::assert_valid_strict;
use liquesco_schema::binary::TBinary;
use liquesco_schema::identifier::Identifier;
use liquesco_schema::structure::{Field, TStruct};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[test]
fn schema1() {
    let mut builder = builder();
    let binary = builder.add(TBinary::try_new(1, 20).unwrap());
    let schema = builder
        .finish(TStruct::default().add(Field::new(Identifier::try_from("bin").unwrap(), binary)));

    assert_valid_strict(
        TestData {
            bin: "hello".as_bytes().to_vec(),
        },
        &schema,
    );
    // maximum length
    assert_valid_strict(
        TestData {
            bin: "hellohellohellohello".as_bytes().to_vec(),
        },
        &schema,
    );
    // minimum length
    assert_valid_strict(
        TestData {
            bin: "h".as_bytes().to_vec(),
        },
        &schema,
    );
    // err: too short
    assert_invalid_strict(
        TestData {
            bin: "".as_bytes().to_vec(),
        },
        &schema,
    );
    // err: too long
    assert_invalid_strict(
        TestData {
            bin: "hellohellohellohellox".as_bytes().to_vec(),
        },
        &schema,
    );
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct TestData {
    #[serde(with = "serde_bytes")]
    bin: Vec<u8>,
}
