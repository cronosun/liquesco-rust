use crate::utils::{assert_err, assert_ok, builder};
use liquesco_parsing::yaml::parse_from_yaml_str;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::schema::DefaultSchema;
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::type_container::DefaultTypeContainer;
use liquesco_schema::types::range::{Inclusion, TRange};
use liquesco_schema::types::seq::TSeq;
use liquesco_schema::types::uint::TUInt;

fn create_schema_given_inclusion() -> DefaultSchema<'static, DefaultTypeContainer<'static>> {
    let mut builder = builder();

    let range_element = builder.add_unwrap(
        "range_element",
        AnyType::UInt(TUInt::try_new(5u32, 150u32).unwrap()),
    );
    let range_value = TRange::new(range_element, Inclusion::StartInclusive, false);
    let range = builder.add_unwrap("range", AnyType::Range(range_value));
    let root = builder.add_unwrap("root", AnyType::Seq(TSeq::try_new(range, 1, 20).unwrap()));

    builder.finish(root).unwrap().into()
}

fn create_schema_supplied_inclusion() -> DefaultSchema<'static, DefaultTypeContainer<'static>> {
    let mut builder = builder();

    let range_element = builder.add_unwrap(
        "range_element",
        AnyType::UInt(TUInt::try_new(5u32, 150u32).unwrap()),
    );
    let range_value = TRange::new(range_element, Inclusion::Supplied, false);
    let range = builder.add_unwrap("range", AnyType::Range(range_value));
    let root = builder.add_unwrap("root", AnyType::Seq(TSeq::try_new(range, 1, 20).unwrap()));

    builder.finish(root).unwrap().into()
}

#[test]
fn ok_1() {
    let schema = create_schema_given_inclusion();
    assert_ok(parse_from_yaml_str(&schema, include_str!("range_ok.yaml")))
}

#[test]
fn err_equal() {
    let schema = create_schema_given_inclusion();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("range_err_equal.yaml"),
    ))
}

#[test]
fn err_start_end_ord() {
    let schema = create_schema_given_inclusion();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("range_err_start_end_ord.yaml"),
    ))
}

#[test]
fn ok_2() {
    let schema = create_schema_supplied_inclusion();
    assert_ok(parse_from_yaml_str(
        &schema,
        include_str!("range_supplied_inclusion_ok.yaml"),
    ))
}
