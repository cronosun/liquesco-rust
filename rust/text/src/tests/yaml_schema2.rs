use crate::tests::id;
use crate::tests::builder::builder;
use liquesco_core::schema::core::Schema;
use liquesco_core::schema::ascii::TAscii;
use liquesco_core::schema::uint::TUInt;
use liquesco_core::schema::seq::TSeq;
use liquesco_core::schema::structure::TStruct;
use liquesco_core::schema::option::TOption;
use crate::yaml::parse_from_yaml_str;
use liquesco_core::schema::any_type::AnyType;
use crate::tests::{assert_ok, assert_err};

fn create_schema() -> impl Schema<'static> {
    let mut builder = builder();
    
    // a structure: a person
    let field_first_name = builder.add(AnyType::Ascii(TAscii::try_new(1, 50, 0, 127).unwrap())); // TODO: Change to unicode type
    let field_last_name = builder.add(AnyType::Ascii(TAscii::try_new(1, 50, 0, 127).unwrap())); // TODO: Change to unicode type
    let field_year_born = builder.add(AnyType::UInt(TUInt::try_new(1000, 3000).unwrap()));
    let email = builder.add(AnyType::Ascii(TAscii::try_new(1, 100, 0, 127).unwrap())); 
    let field_email = builder.add(AnyType::Option(TOption::new(email)));
    let structure = builder.add(
        TStruct::builder()
        .field(id("first_name"), field_first_name)
        .field(id("last_name"), field_last_name)
        .field(id("year_born"), field_year_born)
        .field(id("email"), field_email).build()
    );
    
    // people (structure) within a sequence
    builder.finish(AnyType::Seq(TSeq::try_new(structure, 1, 20).unwrap()))
}

#[test]
fn ok_1() {
    let schema = create_schema();
    assert_ok(parse_from_yaml_str(&schema, include_str!("schema2/working1.yaml")))
}

#[test]
fn unused_field() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(&schema, include_str!("schema2/unused_field.yaml")))
}
