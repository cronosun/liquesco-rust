use crate::utils::{assert_ok, builder};
use liquesco_common::decimal::Decimal;
use liquesco_parsing::yaml::parse_from_yaml_str;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::schema::DefaultSchema;
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::type_container::DefaultTypeContainer;
use liquesco_schema::types::decimal::TDecimal;
use liquesco_schema::types::seq::TSeq;

fn create_schema() -> DefaultSchema<'static, DefaultTypeContainer<'static>> {
    let mut builder = builder();

    let decimal = builder.add_unwrap(
        "decimal",
        TDecimal::try_new(
            Decimal::from_parts(std::i128::MIN + 1, std::i8::MAX),
            Decimal::from_parts(std::i128::MAX - 1, std::i8::MAX),
        )
        .unwrap(),
    );
    let root = builder.add_unwrap(
        "root",
        AnyType::Seq(TSeq::try_new(decimal, 1, 500).unwrap()),
    );

    builder.finish(root).unwrap().into()
}

#[test]
fn ok_1() {
    let schema = create_schema();
    assert_ok(parse_from_yaml_str(&schema, include_str!("working1.yaml")))
}
