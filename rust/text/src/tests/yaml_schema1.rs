use crate::tests::builder::builder;
use crate::tests::{assert_err, assert_ok};
use crate::yaml::parse_from_yaml_str;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::ascii::TAscii;
use liquesco_schema::core::Schema;
use liquesco_schema::doc_type::DocType;
use liquesco_schema::seq::TSeq;

fn create_schema() -> impl Schema<'static> {
    let mut builder = builder();
    let text = builder.add(AnyType::Ascii(DocType::from(
        TAscii::try_new(2, 20, 65, 90).unwrap(),
    )));
    builder.finish(AnyType::Seq(DocType::from(
        TSeq::try_new(text, 3, 6).unwrap(),
    )))
}

#[test]
fn ok_1() {
    let schema = create_schema();
    assert_ok(parse_from_yaml_str(
        &schema,
        include_str!("schema1/input1.yaml"),
    ))
}

#[test]
fn ok_maximum() {
    let schema = create_schema();
    assert_ok(parse_from_yaml_str(
        &schema,
        include_str!("schema1/input2.yaml"),
    ))
}

#[test]
fn err_too_few_elements() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("schema1/to_few_elements.yaml"),
    ))
}

#[test]
fn invalid_ascii_code() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("schema1/invalid_ascii_code.yaml"),
    ))
}
