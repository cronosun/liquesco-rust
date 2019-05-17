use crate::schema::core::Schema;
use crate::schema::seq::Direction;
use crate::schema::seq::TSeq;
use crate::schema::uint::TUInt;
use crate::tests::schema::builder::builder;
use crate::tests::schema::ordering::ord_schema;
use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;
use crate::schema::doc_type::DocType;

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

fn create_schema() -> impl Schema<'static> {
    let mut builder = builder();
    let int = builder.add(DocType::from(TUInt::try_new(50, 100).unwrap()));
    builder.finish(DocType::from(TSeq::try_new(int, 1, 10).unwrap()))
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct WithSequence(Vec<u32>);

fn ordering_create_schema() -> impl Schema<'static> {
    ord_schema(
        |builder| {
            let element = builder.add(DocType::from(TUInt::try_new(0, std::u64::MAX).unwrap()));
            let seq = TSeq::try_new(element, 0, std::u32::MAX).unwrap();
            builder.add(DocType::from(seq))
        },
        Direction::Ascending,
        true,
    )
}

#[test]
fn ordering() {
    let schema = ordering_create_schema();
    // ordering (lex compare)
    assert_valid_strict(
        (
            vec![0usize, 1usize, 2usize, 3usize],
            vec![0usize, 1usize, 2usize, 4usize],
        ),
        &schema,
    );

    // ordering (lex compare): the second list is greater (even tou there are fewer elements)
    assert_valid_strict(
        (
            vec![0usize, 1usize, 2usize, 3usize],
            vec![0usize, 1usize, 3usize],
        ),
        &schema,
    );

    // ordering (lex compare): two lists: The longer wins
    assert_valid_strict(
        (
            vec![0usize, 1usize, 2usize],
            vec![0usize, 1usize, 2usize, 0usize],
        ),
        &schema,
    );

    // invalid: duplicate
    assert_invalid_strict((vec![0usize, 1usize], vec![0usize, 1usize]), &schema);

    // invalid: wrong ordering
    assert_invalid_strict(
        (vec![0usize, 1usize, 0usize], vec![0usize, 1usize]),
        &schema,
    );

    // invalid: wrong ordering
    assert_invalid_strict(
        (vec![0usize, 2usize], vec![0usize, 1usize, 6666usize]),
        &schema,
    );
}
