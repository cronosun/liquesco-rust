use crate::tests::builder::builder;
use crate::tests::id;
use crate::tests::{assert_err, assert_ok};
use crate::yaml::parse_from_yaml_str;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::ascii::TAscii;
use liquesco_schema::core::Schema;
use liquesco_schema::doc_type::DocType;
use liquesco_schema::enumeration::TEnum;
use liquesco_schema::enumeration::Variant;
use liquesco_schema::seq::TSeq;
use liquesco_schema::sint::TSInt;

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

    let enum_value = TEnum::default()
        .add(Variant::new(id("the_empty_variant")))
        .add(Variant::new(id("normal_variant")).add_value(variant1))
        .add(
            Variant::new(id("two_value_variant"))
                .add_value(variant2_1)
                .add_value(variant2_2),
        );

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
