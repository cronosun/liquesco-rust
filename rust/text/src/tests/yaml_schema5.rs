use crate::tests::builder::builder;
use crate::tests::id;
use crate::tests::{assert_err, assert_ok};
use crate::yaml::parse_from_yaml_str;
use liquesco_core::schema::anchors::TAnchors;
use liquesco_core::schema::any_type::AnyType;
use liquesco_core::schema::core::Schema;
use liquesco_core::schema::doc_type::DocType;
use liquesco_core::schema::option::TOption;
use liquesco_core::schema::reference::TReference;
use liquesco_core::schema::structure::TStruct;
use liquesco_core::schema::unicode::{LengthType, TUnicode};

/// anchors and references
fn create_schema() -> impl Schema<'static> {
    let mut builder = builder();

    let field_text = builder.add(AnyType::Unicode(DocType::from(
        TUnicode::try_new(0, 500, LengthType::Byte).unwrap(),
    )));
    // this is required
    let field_next1 = builder.add(AnyType::Reference(DocType::from(TReference)));
    // optionally a second reference
    let field_next2_present = builder.add(AnyType::Reference(DocType::from(TReference)));
    let field_next2 = builder.add(AnyType::Option(DocType::from(TOption::new(
        field_next2_present,
    ))));

    let struct_value = TStruct::builder()
        .field(id("text"), field_text)
        .field(id("next1"), field_next1)
        .field(id("next2"), field_next2)
        .build();

    let structure = builder.add(AnyType::Struct(struct_value.into()));

    // now wrap that inside anchors
    builder.finish(AnyType::Anchors(DocType::from(TAnchors::new(
        structure, structure,
    ))))
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
