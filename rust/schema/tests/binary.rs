mod common;

use common::builder::builder;
use common::utils::assert_invalid_strict;
use common::utils::assert_valid_strict;
use liquesco_schema::identifier::Identifier;
use liquesco_schema::schema::DefaultSchema;
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::type_container::DefaultTypeContainer;
use liquesco_schema::types::binary::TBinary;
use liquesco_schema::types::structure::{Field, TStruct};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[test]
fn schema1() {
    let mut builder = builder();
    let binary = builder.add_unwrap("binary", TBinary::try_new(1, 20).unwrap());
    let root = builder.add_unwrap(
        "root",
        TStruct::default().add(Field::new(Identifier::try_from("bin").unwrap(), binary)),
    );
    let schema: DefaultSchema<'static, DefaultTypeContainer<'static>> =
        builder.finish(root).unwrap().into();

    assert_valid_strict(
        TestData {
            bin: b"hello".to_vec(),
        },
        &schema,
    );
    // maximum length
    assert_valid_strict(
        TestData {
            bin: b"hellohellohellohello".to_vec(),
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
    assert_invalid_strict(TestData { bin: b"".to_vec() }, &schema);
    // err: too long
    assert_invalid_strict(
        TestData {
            bin: b"hellohellohellohellox".to_vec(),
        },
        &schema,
    );
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct TestData {
    #[serde(with = "serde_bytes")]
    bin: Vec<u8>,
}
