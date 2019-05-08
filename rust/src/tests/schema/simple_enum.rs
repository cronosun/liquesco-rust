use crate::schema::vascii::VAscii;
use crate::schema::venum::VEnum;
use crate::schema::vuint::VUInt;
use crate::tests::schema::builder::builder;
use crate::tests::schema::utils::id;

use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;
use serde::{Deserialize, Serialize};

#[test]
fn schema1() {
    let mut builder = builder();
    let int = builder.add(VUInt::try_new(1, 200).unwrap());
    let upper_case = builder.add(VAscii::try_new(2, 10, 65, 90).unwrap());
    let schema = builder.finish(
        VEnum::builder()
            .empty_variant(id("shutdown"))
            .variant(id("add"), int)
            .variant(id("delete_account"), upper_case)
            .build(),
    );

    // valid
    assert_valid_strict(Schema1Enum::Shutdown, &schema);
    assert_valid_strict(Schema1Enum::Add(45), &schema);
    assert_valid_strict(Schema1Enum::DeleteAccount("MYACCOUNT".to_string()), &schema);

    // invalid
    assert_invalid_strict(Schema1Enum::Add(201), &schema);
    assert_invalid_strict(
        Schema1Enum::DeleteAccount("MYACCOUNTXX".to_string()),
        &schema,
    );
    assert_invalid_strict(Schema1EnumTooManyFields::AdditionalField, &schema);
    assert_invalid_strict(Schema1EnumTooManyValues::Add(45, 45), &schema);
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum Schema1Enum {
    Shutdown,
    Add(u64),
    DeleteAccount(String),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum Schema1EnumTooManyFields {
    Shutdown,
    Add(u64),
    DeleteAccount(String),
    AdditionalField,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum Schema1EnumTooManyValues {
    Shutdown,
    Add(u64, u64),
    DeleteAccount(String),
}
