use crate::tests::builder::builder;
use crate::tests::id;
use crate::tests::{assert_err, assert_ok};
use crate::yaml::parse_from_yaml_str;
use liquesco_schema::anchors::TAnchors;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::Schema;
use liquesco_schema::option::TOption;
use liquesco_schema::reference::TReference;
use liquesco_schema::structure::Field;
use liquesco_schema::structure::TStruct;
use liquesco_schema::unicode::{LengthType, TUnicode};

/// anchors and references
fn create_schema() -> impl Schema<'static> {
    let mut builder = builder();

    let field_text = builder.add(AnyType::Unicode(
        TUnicode::try_new(0, 500, LengthType::Byte).unwrap(),
    ));
    // this is required
    let field_next1 = builder.add(AnyType::Reference(TReference::default()));
    // optionally a second reference
    let field_next2_present = builder.add(AnyType::Reference(TReference::default()));
    let field_next2 = builder.add(AnyType::Option(TOption::new(
        field_next2_present,
    )));

    let struct_value = TStruct::default()
        .add(Field::new(id("text"), field_text))
        .add(Field::new(id("next1"), field_next1))
        .add(Field::new(id("next2"), field_next2));

    let structure = builder.add(AnyType::Struct(struct_value.into()));

    // now wrap that inside anchors
    builder.finish(AnyType::Anchors(TAnchors::new(
        structure, structure,
    )))
}

#[test]
fn ok_1() {
    let schema = create_schema();
    assert_ok(parse_from_yaml_str(
        &schema,
        include_str!("schema5/working1.yaml"),
    ))
}

#[test]
fn ok_order_does_not_matter() {
    let schema = create_schema();
    assert_ok(parse_from_yaml_str(
        &schema,
        include_str!("schema5/order_does_not_matter.yaml"),
    ))
}

#[test]
fn err_unused() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("schema5/unused_item.yaml"),
    ))
}

#[test]
fn err_unknown_ref() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("schema5/unknown_ref.yaml"),
    ))
}
