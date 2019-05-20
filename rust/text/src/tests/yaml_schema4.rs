use crate::tests::builder::builder;
use crate::tests::id;
use crate::tests::{assert_err, assert_ok};
use crate::yaml::parse_from_yaml_str;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::Schema;
use liquesco_schema::doc_type::DocType;
use liquesco_schema::float::TFloat32;
use liquesco_schema::float::TFloat64;
use liquesco_schema::seq::TSeq;
use liquesco_schema::structure::Field;
use liquesco_schema::structure::TStruct;

/// Creates an enum
fn create_schema() -> impl Schema<'static> {
    let mut builder = builder();

    let field1 = builder.add(AnyType::Float32(DocType::from(
        TFloat32::try_new(std::f32::MIN.into(), std::f32::MAX.into()).unwrap(),
    )));
    let field2 = builder.add(AnyType::Float64(DocType::from(
        TFloat64::try_new(std::f64::MIN.into(), std::f64::MAX.into()).unwrap(),
    )));

    let struct_value = TStruct::default()
        .add(Field::new(id("my_float_32"), field1))
        .add(Field::new(id("my_float_64"), field2));

    let structure = builder.add(AnyType::Struct(struct_value.into()));

    // people (structure) within a sequence
    builder.finish(AnyType::Seq(DocType::from(
        TSeq::try_new(structure, 1, 20).unwrap(),
    )))
}

#[test]
fn ok_1() {
    let schema = create_schema();
    assert_ok(parse_from_yaml_str(
        &schema,
        include_str!("schema4/working1.yaml"),
    ))
}

#[test]
fn invalid_float() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("schema4/invalid_float.yaml"),
    ))
}

#[test]
fn precision_lost() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("schema4/precision_lost.yaml"),
    ))
}
