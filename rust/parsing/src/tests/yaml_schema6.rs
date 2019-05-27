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
use liquesco_schema::range::{TRange, Inclusion};
use liquesco_schema::uint::TUInt;

fn create_schema_given_inclusion() -> impl Schema<'static> {
    let mut builder = builder();

    let range_element = builder.add(AnyType::UInt(DocType::from(
        TUInt::try_new(5, 150).unwrap(),
    )));

    let range_value = TRange {
        element: range_element,
        inclusion: Inclusion::BothInclusive,
        allow_equal: false
    };

    let range = builder.add(AnyType::Range(range_value.into()));

    builder.finish(AnyType::Seq(DocType::from(
        TSeq::try_new(range, 1, 20).unwrap(),
    )))
}

fn create_schema_supplied_inclusion() -> impl Schema<'static> {
    let mut builder = builder();

    let range_element = builder.add(AnyType::UInt(DocType::from(
        TUInt::try_new(5, 150).unwrap(),
    )));

    let range_value = TRange {
        element: range_element,
        inclusion: Inclusion::Supplied,
        allow_equal: false
    };

    let range = builder.add(AnyType::Range(range_value.into()));

    builder.finish(AnyType::Seq(DocType::from(
        TSeq::try_new(range, 1, 20).unwrap(),
    )))
}

#[test]
fn ok_1() {
    let schema = create_schema_given_inclusion();
    assert_ok(parse_from_yaml_str(
        &schema,
        include_str!("schema6/range_ok.yaml"),
    ))
}

#[test]
fn err_equal() {
    let schema = create_schema_given_inclusion();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("schema6/range_err_equal.yaml"),
    ))
}

#[test]
fn err_start_end_ord() {
    let schema = create_schema_given_inclusion();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("schema6/range_err_start_end_ord.yaml"),
    ))
}

#[test]
fn ok_2() {
    let schema = create_schema_supplied_inclusion();
    assert_ok(parse_from_yaml_str(
        &schema,
        include_str!("schema6/range_supplied_inclusion_ok.yaml"),
    ))
}