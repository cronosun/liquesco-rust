use crate::schema::core::Schema;
use crate::schema::core::ValidatorRef;
use crate::schema::validators::AnyValidator;
use crate::schema::vseq::Direction;
use crate::schema::vseq::Ordering;
use crate::schema::vseq::VSeq;
use crate::tests::schema::builder::builder;
use crate::tests::schema::builder::Builder;
use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;
use std::fmt::Debug;

pub fn ord_assert_equal<V, S>(validator: V, item1: S, item2: S)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static + Clone,
    V: Into<AnyValidator<'static>>,
{
    // we validate twice: Once with unique=true and once with unique=false
    // ... one of them should fail, one succeed
    let validator_orig = validator.into();
    let schema_u = ord_schema_single(validator_orig.clone(), Direction::Descending, true);
    let schema = ord_schema_single(validator_orig, Direction::Descending, false);

    assert_invalid_strict((item1.clone(), item2.clone()), &schema_u);
    assert_valid_strict((item1, item2), &schema);
}

pub fn ord_assert_ascending<V, S>(validator: V, item1: S, item2: S)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    V: Into<AnyValidator<'static>>,
{
    // unique = true; strictly ascending
    let schema = ord_schema_single(validator, Direction::Ascending, true);
    assert_valid_strict((item1, item2), &schema);
}

pub fn ord_assert_given_schema<TSchema, S>(schema: &TSchema, item1: S, item2: S)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    TSchema : Schema,
{
    assert_valid_strict((item1, item2), schema);
}

pub fn ord_schema<FElement: FnOnce(&mut Builder<'static>) -> ValidatorRef>(
    element: FElement,
    direction: Direction,
    unique: bool,
) -> impl Schema {
    let mut builder = builder();
    let element_ref = element(&mut builder);

    let mut vseq = VSeq::try_new(element_ref, 0, std::u32::MAX).unwrap();
    vseq.ordering = Ordering::Sorted { direction, unique };

    builder.finish(vseq)
}

fn ord_schema_single<'a, V>(validator: V, direction: Direction, unique: bool) -> impl Schema
where
    V: Into<AnyValidator<'static>>,
{
    ord_schema(|builder| builder.add(validator.into()), direction, unique)
}
