#![allow(dead_code)]

use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::Schema;
use liquesco_schema::core::TypeRef;
use liquesco_schema::doc_type::DocType;
use liquesco_schema::seq::Direction;
use liquesco_schema::seq::Ordering;
use liquesco_schema::seq::TSeq;
use crate::common::builder::builder;
use crate::common::builder::Builder;
use crate::common::utils::assert_invalid_strict;
use crate::common::utils::assert_valid_strict;
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

pub fn ord_assert_given_schema<'a, TSchema, S>(schema: &TSchema, item1: S, item2: S)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    TSchema: Schema<'a>,
{
    assert_valid_strict((item1, item2), schema);
}

pub fn ord_schema<FElement: FnOnce(&mut Builder<'static>) -> TypeRef>(
    element: FElement,
    direction: Direction,
    unique: bool,
) -> impl Schema<'static> {
    let mut builder = builder();
    let element_ref = element(&mut builder);

    let mut seq = TSeq::try_new(element_ref, 0, std::u32::MAX).unwrap();
    seq.ordering = Ordering::Sorted { direction, unique };

    builder.finish(DocType::from(seq))
}

fn ord_schema_single<'a, T>(any_type: T, direction: Direction, unique: bool) -> impl Schema<'static>
where
    T: Into<AnyType<'static>>,
{
    ord_schema(|builder| builder.add(any_type.into()), direction, unique)
}
