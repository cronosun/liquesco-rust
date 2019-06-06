use liquesco_schema::schema::DefaultSchema;
use crate::utils::{assert_err, assert_ok, builder, finish};
use liquesco_parsing::yaml::parse_from_yaml_str;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::ascii::TAscii;
use liquesco_schema::identifier::Identifier;
use liquesco_schema::seq::TSeq;
use liquesco_schema::unicode::TUnicode;
use liquesco_schema::unicode::LengthType;
use liquesco_schema::map::TMap;
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::type_container::DefaultTypeContainer;
use liquesco_schema::schema_builder::BuildsOwnSchema;

/// It's a map of "identifier" -> "string"
fn create_schema() -> DefaultSchema<'static, DefaultTypeContainer<'static>> {
    let mut builder = builder();
    let key = Identifier::build_schema(&mut builder);
    let value = builder.add_unwrap("value", AnyType::Unicode(TUnicode::try_new(0, 50, LengthType::Utf8Byte).unwrap()));
    let root = builder.add_unwrap("root", AnyType::Map(TMap::new(key, value)));
    finish(builder, root)
}

#[test]
fn ok_1() {
    let schema = create_schema();
    assert_ok(parse_from_yaml_str(&schema, include_str!("ok_simple.yaml")))
}
