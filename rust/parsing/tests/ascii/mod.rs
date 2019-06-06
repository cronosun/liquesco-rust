use liquesco_schema::schema::DefaultSchema;
use crate::utils::{assert_err, assert_ok, builder, finish};
use liquesco_parsing::yaml::parse_from_yaml_str;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::ascii::TAscii;
use liquesco_schema::seq::TSeq;
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::type_container::DefaultTypeContainer;

fn create_schema() -> DefaultSchema<'static, DefaultTypeContainer<'static>> {
    let mut builder = builder();
    let text = builder.add_unwrap("ascii",AnyType::Ascii(TAscii::try_new(2, 20, 65, 90).unwrap()));
    let root = builder.add_unwrap("root", AnyType::Seq(TSeq::try_new(text, 3, 6).unwrap()));
    finish(builder, root)
}

#[test]
fn ok_1() {
    let schema = create_schema();
    assert_ok(parse_from_yaml_str(&schema, include_str!("input1.yaml")))
}

#[test]
fn ok_maximum() {
    let schema = create_schema();
    assert_ok(parse_from_yaml_str(&schema, include_str!("input2.yaml")))
}

#[test]
fn err_too_few_elements() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("to_few_elements.yaml"),
    ))
}

#[test]
fn invalid_ascii_code() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("invalid_ascii_code.yaml"),
    ))
}
