use crate::tests::builder::builder;
use crate::tests::id;
use crate::tests::{assert_err, assert_ok};
use crate::yaml::parse_from_yaml_str;
use liquesco_core::schema::any_type::AnyType;
use liquesco_core::schema::core::Schema;
use liquesco_core::schema::float::TFloat32;
use liquesco_core::schema::float::TFloat64;
use liquesco_core::schema::seq::TSeq;
use liquesco_core::schema::structure::TStruct;

/// Creates an enum
fn create_schema() -> impl Schema<'static> {
    let mut builder = builder();

    let field1 = builder.add(AnyType::Float32(
        TFloat32::try_new(std::f32::MIN, std::f32::MAX).unwrap(),
    ));
    let field2 = builder.add(AnyType::Float64(
        TFloat64::try_new(std::f64::MIN, std::f64::MAX).unwrap(),
    ));

    let struct_value = TStruct::builder()
        .field(id("my_float_32"), field1)
        .field(id("my_float_64"), field2)
        .build();

    let structure = builder.add(AnyType::Struct(struct_value));

    // people (structure) within a sequence
    builder.finish(AnyType::Seq(TSeq::try_new(structure, 1, 20).unwrap()))
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
