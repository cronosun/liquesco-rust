#![allow(dead_code)]

use crate::common::builder::builder;
use crate::common::utils::assert_invalid_strict;
use crate::common::utils::assert_valid_strict;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::Schema;
use liquesco_schema::core::TypeRef;
use liquesco_schema::schema::DefaultSchema;
use liquesco_schema::schema_builder::DefaultSchemaBuilder;
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::type_container::DefaultTypeContainer;
use liquesco_schema::types::seq::TSeq;
use liquesco_schema::types::seq::{Direction, Sorted};
use std::fmt::Debug;

pub fn ord_assert_equal<T, S>(any_type: T, item1: S, item2: S)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static + Clone,
    T: Into<AnyType<'static>>,
{
    // we validate twice: Once with unique=true and once with unique=false
    // ... one of them should fail, one succeed
    let type_orig = any_type.into();
    let schema_u = ord_schema_single(type_orig.clone(), Direction::Descending, true);
    let schema = ord_schema_single(type_orig, Direction::Descending, false);

    assert_invalid_strict((item1.clone(), item2.clone()), &schema_u);
    assert_valid_strict((item1, item2), &schema);
}

pub fn ord_assert_ascending<T, S>(any_type: T, item1: S, item2: S)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    T: Into<AnyType<'static>>,
{
    // unique = true; strictly ascending
    let schema = ord_schema_single(any_type, Direction::Ascending, true);
    assert_valid_strict((item1, item2), &schema);
}

pub fn ord_assert_given_schema<TSchema, S>(schema: &TSchema, item1: S, item2: S)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    TSchema: Schema,
{
    assert_valid_strict((item1, item2), schema);
}

pub fn ord_schema<FElement: FnOnce(&mut DefaultSchemaBuilder<'static>) -> TypeRef>(
    element: FElement,
    direction: Direction,
    unique: bool,
) -> DefaultSchema<'static, DefaultTypeContainer<'static>> {
    let mut builder = builder();
    let element_ref = element(&mut builder);

    let seq = TSeq::try_new(element_ref, 0, std::u32::MAX)
        .unwrap()
        .with_sorted(Sorted { direction, unique });
    let root = builder.add_unwrap("root", seq);

    builder.finish(root).unwrap().into()
}

fn ord_schema_single<T>(any_type: T, direction: Direction, unique: bool) -> impl Schema
where
    T: Into<AnyType<'static>>,
{
    ord_schema(
        |builder| builder.add_unwrap("ord_element", any_type.into()),
        direction,
        unique,
    )
}
