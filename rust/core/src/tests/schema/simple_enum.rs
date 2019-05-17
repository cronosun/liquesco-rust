use crate::schema::core::Schema;
use crate::schema::ascii::TAscii;
use crate::schema::enumeration::TEnum;
use crate::schema::seq::Direction;
use crate::schema::uint::TUInt;
use crate::tests::schema::builder::builder;
use crate::tests::schema::ordering::ord_schema;
use crate::tests::schema::utils::assert_valid_extended;
use crate::tests::schema::utils::id;
use crate::schema::doc_type::DocType;

use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;
use serde::{Deserialize, Serialize};

#[test]
fn schema1() {
    let mut builder = builder();
    let int = builder.add(DocType::from(TUInt::try_new(1, 200).unwrap()));
    let upper_case = builder.add(DocType::from(TAscii::try_new(2, 10, 65, 90).unwrap()));
    let schema = builder.finish(
        DocType::from(TEnum::builder()
            .empty_variant(id("shutdown"))
            .variant(id("add"), int)
            .variant(id("delete_account"), upper_case)
            .build()),
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

fn ordering_create_schema() -> impl Schema<'static> {
    ord_schema(
        |builder| {
            let variant1_type = builder.add(DocType::from(TUInt::try_new(0, std::u64::MAX).unwrap()));
            builder.add(
                DocType::from(TEnum::builder()
                    .variant(id("variant1"), variant1_type)
                    .empty_variant(id("variant2"))
                    .build()),
            )
        },
        Direction::Ascending,
        true,
    )
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum Enum1ForOrdering {
    Variant1(usize),
    Variant2,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum Enum1ForOrderingExt {
    Variant1(usize, usize, String),
    Variant2(String),
}

#[test]
fn ordering_no_extension() {
    let schema = ordering_create_schema();

    // variant ordinals are compared first (so variant1 is always < variant2)
    assert_valid_strict(
        (Enum1ForOrdering::Variant1(158), Enum1ForOrdering::Variant2),
        &schema,
    );

    assert_valid_strict(
        (
            Enum1ForOrdering::Variant1(10),
            Enum1ForOrdering::Variant1(11),
        ),
        &schema,
    );

    // duplicates
    assert_invalid_strict(
        (
            Enum1ForOrdering::Variant1(158),
            Enum1ForOrdering::Variant1(158),
        ),
        &schema,
    );

    // duplicates
    assert_invalid_strict(
        (Enum1ForOrdering::Variant2, Enum1ForOrdering::Variant2),
        &schema,
    );

    // wrong ordering
    assert_invalid_strict(
        (Enum1ForOrdering::Variant2, Enum1ForOrdering::Variant1(158)),
        &schema,
    );

    // wrong ordering
    assert_invalid_strict(
        (
            Enum1ForOrdering::Variant1(159),
            Enum1ForOrdering::Variant1(158),
        ),
        &schema,
    );
}

#[test]
fn ordering_extension() {
    let schema = ordering_create_schema();

    // we still can compare those two things (even if one has extended data)
    assert_valid_extended(
        (
            Enum1ForOrdering::Variant1(158),
            Enum1ForOrderingExt::Variant2("this is ignored".to_string()),
        ),
        &schema,
    );

    assert_valid_extended(
        (
            Enum1ForOrderingExt::Variant1(158, 44444, "ignored".to_string()),
            Enum1ForOrdering::Variant1(159),
        ),
        &schema,
    );

    // duplicates: There's not byte-wise duplicates, but the data in "ext" is
    // ignored since not defined in the schema.
    assert_invalid_strict(
        (
            Enum1ForOrderingExt::Variant1(159, 232, "ignored".to_string()),
            Enum1ForOrdering::Variant1(159),
        ),
        &schema,
    );

    // duplicates
    assert_invalid_strict(
        (
            Enum1ForOrderingExt::Variant2("ignored".to_string()),
            Enum1ForOrdering::Variant2,
        ),
        &schema,
    );

    // wrong ordering
    assert_invalid_strict(
        (
            Enum1ForOrdering::Variant1(159),
            Enum1ForOrderingExt::Variant1(158, 232, "ignored".to_string()),
        ),
        &schema,
    );
}
