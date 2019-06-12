use crate::utils::{assert_err, assert_ok, builder, id};
use liquesco_parsing::yaml::parse_from_yaml_str;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::schema::DefaultSchema;
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::type_container::DefaultTypeContainer;
use liquesco_schema::types::ascii::TAscii;
use liquesco_schema::types::option::TOption;
use liquesco_schema::types::seq::TSeq;
use liquesco_schema::types::structure::Field;
use liquesco_schema::types::structure::TStruct;
use liquesco_schema::types::uint::TUInt;
use liquesco_schema::types::unicode::LengthType;
use liquesco_schema::types::unicode::TUnicode;

fn create_schema() -> DefaultSchema<'static, DefaultTypeContainer<'static>> {
    let mut builder = builder();

    // a structure: a person
    let field_first_name = builder.add_unwrap(
        "first_name",
        AnyType::Unicode(TUnicode::try_new(1, 100, LengthType::Byte).unwrap()),
    );
    let field_last_name = builder.add_unwrap(
        "last_name",
        AnyType::Unicode(TUnicode::try_new(1, 100, LengthType::Byte).unwrap()),
    );
    let field_year_born = builder.add_unwrap(
        "year_born",
        AnyType::UInt(TUInt::try_new(1000u32, 3000u32).unwrap()),
    );
    let email = builder.add_unwrap(
        "email",
        AnyType::Ascii(TAscii::try_new(1, 100, 0, 127).unwrap()),
    );
    let field_email = builder.add_unwrap("maybe_email", AnyType::Option(TOption::new(email)));

    let struct_type = TStruct::default()
        .add(Field::new(id("first_name"), field_first_name))
        .add(Field::new(id("last_name"), field_last_name))
        .add(Field::new(id("year_born"), field_year_born))
        .add(Field::new(id("email"), field_email));

    let structure = builder.add_unwrap("structure", struct_type);
    let root = builder.add_unwrap(
        "root",
        AnyType::Seq(TSeq::try_new(structure, 1, 20).unwrap()),
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
fn unused_field() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("unused_field.yaml"),
    ))
}
