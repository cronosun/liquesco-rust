use crate::utils::{assert_err, assert_ok, builder};
use liquesco_parsing::yaml::parse_from_yaml_str;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::schema::DefaultSchema;
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::type_container::DefaultTypeContainer;
use liquesco_schema::types::binary::TBinary;
use liquesco_schema::types::seq::TSeq;

fn create_schema() -> DefaultSchema<'static, DefaultTypeContainer<'static>> {
    let mut builder = builder();
    let binary = TBinary::try_new(0, 20).unwrap();
    let binary = builder.add_unwrap("binary", AnyType::Binary(binary));
    let root = builder.add_unwrap("root", AnyType::Seq(TSeq::try_new(binary, 1, 20).unwrap()));
    builder.finish(root).unwrap().into()
}

#[test]
fn ok_1() {
    let schema = create_schema();
    assert_ok(parse_from_yaml_str(&schema, include_str!("working1.yaml")))
}

#[test]
fn err_too_long_base64() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("err_too_long_base64.yaml"),
    ))
}

#[test]
fn err_too_long_hex() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("err_too_long_hex.yaml"),
    ))
}

#[test]
fn err_too_long_seq() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("err_too_long_seq.yaml"),
    ))
}
