use crate::utils::{assert_err, assert_ok, builder, finish};
use liquesco_parsing::yaml::parse_from_yaml_str;
use liquesco_schema::identifier::Identifier;
use liquesco_schema::schema::DefaultSchema;
use liquesco_schema::schema_builder::BuildsOwnSchema;
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::type_container::DefaultTypeContainer;
use liquesco_schema::types::key_ref::TKeyRef;
use liquesco_schema::types::map::TMap;
use liquesco_schema::types::root_map::TRootMap;
use liquesco_schema::types::seq::TSeq;
use liquesco_schema::types::structure::{Field, TStruct};
use liquesco_schema::types::uint::TUInt;
use std::convert::TryFrom;

/// We have two nested maps: A root map (int -> inner_map) and an inner map: (identifier -> value).
fn create_schema() -> DefaultSchema<'static, DefaultTypeContainer<'static>> {
    let mut builder = builder();

    let outer_ref = builder.add_unwrap("outer_ref", TKeyRef::default().with_level(1));
    let outers_type = builder.add_unwrap("outer_refs", TSeq::try_new(outer_ref, 0, 5).unwrap());
    let inner_ref = builder.add_unwrap("inner_ref", TKeyRef::default());
    let inners_type = builder.add_unwrap("inner_refs", TSeq::try_new(inner_ref, 0, 5).unwrap());

    // value
    let value = builder.add_unwrap(
        "value",
        TStruct::default()
            .add(Field::new(
                Identifier::try_from("outers").unwrap(),
                outers_type,
            ))
            .add(Field::new(
                Identifier::try_from("inners").unwrap(),
                inners_type,
            )),
    );

    // Inner map
    let identifier = Identifier::build_schema(&mut builder);
    let inner_map =
        builder.add_unwrap("inner_map", TMap::new(identifier, value).with_anchors(true));

    // outer map
    // note: we can reference own keys in root type.
    let root_type = builder.add_unwrap("root_type", TKeyRef::default());
    let outer_map_key = builder.add_unwrap("outer_map_key", TUInt::try_new(0u32, 100u32).unwrap());
    let outer_map = builder.add_unwrap(
        "outer_map",
        TRootMap::new(root_type, outer_map_key, inner_map),
    );

    finish(builder, outer_map)
}

#[test]
fn ok_1() {
    let schema = create_schema();
    assert_ok(parse_from_yaml_str(&schema, include_str!("ok1.yaml")))
}

#[test]
fn err_root_key_not_found() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("err_root_key_not_found.yaml"),
    ))
}

#[test]
fn err_inner_key_not_found() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("err_inner_key_not_found.yaml"),
    ))
}

#[test]
fn err_too_many_refs() {
    let schema = create_schema();
    assert_err(parse_from_yaml_str(
        &schema,
        include_str!("err_too_many_refs.yaml"),
    ))
}
