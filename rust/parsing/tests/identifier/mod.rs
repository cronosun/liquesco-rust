use crate::utils::{assert_err, assert_ok, builder};
use liquesco_common::ine_range::U64IneRange;
use liquesco_parsing::yaml::parse_from_yaml_str;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::ascii::CodeRange;
use liquesco_schema::ascii::TAscii;
use liquesco_schema::seq::TSeq;
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::schema::DefaultSchema;
use liquesco_schema::type_container::DefaultTypeContainer;

fn create_identifier_schema() -> DefaultSchema<'static, DefaultTypeContainer<'static>> {
    let mut builder = builder();
    let ascii = builder.add_unwrap("ascii",AnyType::Ascii(TAscii::new(
        U64IneRange::try_new("", 0, 10).unwrap(),
        CodeRange::try_new(97, 123).unwrap(),
    )));
    let identifier = builder.add_unwrap("identifier",AnyType::Seq(TSeq::try_new(ascii, 1, 8).unwrap()));
    let root = builder.add_unwrap("root", AnyType::Seq(TSeq::try_new(identifier, 1, 100).unwrap()));
    builder.finish(root).unwrap().into()
}

#[test]
fn ok_1() {
    let schema = create_identifier_schema();
    assert_ok(parse_from_yaml_str(
        &schema,
        include_str!("ok_identifier.yaml"),
    ))
}

#[test]
fn err_one_element_too_long() {
    let schema = create_identifier_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("err_segment_too_long.yaml"),
    ))
}
