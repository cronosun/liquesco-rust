use crate::schema::core::Schema;
use crate::schema::vseq::VSeq;
use crate::schema::vuint::VUInt;
use crate::tests::schema::builder::builder;
use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;

use serde::{Deserialize, Serialize};

#[test]
fn working_1() {
    let schema = create_schema();
    assert_valid_strict(WithSequence(vec![50, 60, 70, 80, 90, 100]), &schema);
}

#[test]
fn working_min_len_1() {
    let schema = create_schema();
    assert_valid_strict(WithSequence(vec![50]), &schema);
}

#[test]
fn working_min_len_2() {
    let schema = create_schema();
    assert_valid_strict(WithSequence(vec![100]), &schema);
}

#[test]
fn working_max_len() {
    let schema = create_schema();
    assert_valid_strict(
        WithSequence(vec![50, 60, 70, 80, 90, 100, 50, 60, 70, 80]),
        &schema,
    );
}

#[test]
fn one_invalid_elements() {
    let schema = create_schema();
    assert_invalid_strict(
        WithSequence(
            // 101 is invalid
            vec![50, 60, 70, 101, 90, 100],
        ),
        &schema,
    );
}

#[test]
fn multiple_invalid_elements() {
    let schema = create_schema();
    assert_invalid_strict(
        WithSequence(
            // 49 & 101 is invalid
            vec![50, 49, 70, 101, 90, 100],
        ),
        &schema,
    );
}

#[test]
fn too_few_elements() {
    let schema = create_schema();
    assert_invalid_strict(
        WithSequence(
            // need at least one element
            vec![],
        ),
        &schema,
    );
}

#[test]
fn too_many_elements() {
    let schema = create_schema();
    assert_invalid_strict(
        WithSequence(
            // 11 elements; 10 elements max
            vec![50, 60, 70, 80, 90, 100, 50, 60, 70, 80, 90],
        ),
        &schema,
    );
}

fn create_schema() -> impl Schema {
    let mut builder = builder();
    let int = builder.add(VUInt::try_new(50, 100).unwrap());
    builder.finish(VSeq::try_new(int, 1, 10).unwrap())
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct WithSequence(Vec<u32>);
