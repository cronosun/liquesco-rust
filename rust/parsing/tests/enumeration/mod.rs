use crate::utils::{assert_err, assert_ok, builder, id};
use liquesco_parsing::yaml::parse_from_yaml_str;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::ascii::TAscii;
use liquesco_schema::enumeration::TEnum;
use liquesco_schema::enumeration::Variant;
use liquesco_schema::schema::DefaultSchema;
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::seq::TSeq;
use liquesco_schema::sint::TSInt;
use liquesco_schema::type_container::DefaultTypeContainer;

/// Creates an enum
fn create_schema() -> DefaultSchema<'static, DefaultTypeContainer<'static>> {
    let mut builder = builder();

    let variant1 = builder.add_unwrap(
        "variant1",
        AnyType::Ascii(TAscii::try_new(1, 50, 0, 127).unwrap()),
    );
    let variant2_1 = builder.add_unwrap(
        "variant21",
        AnyType::Ascii(TAscii::try_new(1, 50, 0, 127).unwrap()),
    );
    let variant2_2 = builder.add_unwrap(
        "variant22",
        AnyType::SInt(TSInt::try_new(-10, 3000).unwrap()),
    );

    let enum_value = TEnum::default()
        .add_variant(Variant::new(id("the_empty_variant")))
        .add_variant(Variant::new(id("normal_variant")).add_value(variant1))
        .add_variant(
            Variant::new(id("two_value_variant"))
                .add_value(variant2_1)
                .add_value(variant2_2),
        );
    let enumeration = builder.add_unwrap("enum", AnyType::Enum(enum_value));
    let root = builder.add_unwrap(
        "root",
        AnyType::Seq(TSeq::try_new(enumeration, 1, 20).unwrap()),
    );
    // people (structure) within a sequence
    builder.finish(root).unwrap().into()
}

#[test]
fn ok_1() {
    let schema = create_schema();
    assert_ok(parse_from_yaml_str(&schema, include_str!("working1.yaml")))
}

#[test]
fn too_many_values() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("too_many_values.yaml"),
    ))
}

#[test]
fn not_enough_values() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("not_enough_values.yaml"),
    ))
}
