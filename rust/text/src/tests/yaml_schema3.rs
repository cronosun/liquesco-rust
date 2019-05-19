use crate::tests::builder::builder;
use crate::tests::id;
use crate::tests::{assert_err, assert_ok};
use crate::yaml::parse_from_yaml_str;
use liquesco_core::schema::any_type::AnyType;
use liquesco_core::schema::ascii::TAscii;
use liquesco_core::schema::core::Schema;
use liquesco_core::schema::doc_type::DocType;
use liquesco_core::schema::enumeration::TEnum;
use liquesco_core::schema::seq::TSeq;
use liquesco_core::schema::sint::TSInt;

/// Creates an enum
fn create_schema() -> impl Schema<'static> {
    let mut builder = builder();

    let variant1 = builder.add(AnyType::Ascii(DocType::from(
        TAscii::try_new(1, 50, 0, 127).unwrap(),
    )));
    let variant2_1 = builder.add(AnyType::Ascii(DocType::from(
        TAscii::try_new(1, 50, 0, 127).unwrap(),
    )));
    let variant2_2 = builder.add(AnyType::SInt(DocType::from(
        TSInt::try_new(-10, 3000).unwrap(),
    )));

    let enum_value = TEnum::builder()
        .empty_variant(id("the_empty_variant"))
        .variant(id("normal_variant"), variant1)
        .multi_variant(id("two_value_variant"), vec![variant2_1, variant2_2].into())
        .build();

    let enumeration = builder.add(AnyType::Enum(DocType::from(enum_value)));

    // people (structure) within a sequence
    builder.finish(AnyType::Seq(DocType::from(
        TSeq::try_new(enumeration, 1, 20).unwrap(),
    )))
}

#[test]
fn ok_1() {
    let schema = create_schema();
    assert_ok(parse_from_yaml_str(
        &schema,
        include_str!("schema3/working1.yaml"),
    ))
}

#[test]
fn too_many_values() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("schema3/too_many_values.yaml"),
    ))
}

#[test]
fn not_enough_values() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("schema3/not_enough_values.yaml"),
    ))
}
