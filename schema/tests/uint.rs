mod common;

use common::ordering::ord_assert_ascending;
use common::ordering::ord_assert_equal;
use common::utils::assert_invalid_strict;
use common::utils::assert_valid_strict;
use common::utils::single_schema;
use liquesco_schema::types::uint::TUInt;

#[test]
fn schema1() {
    let schema = single_schema(TUInt::try_new(5u32, 158u32).unwrap());

    // some valid items
    assert_valid_strict(5usize, &schema);
    assert_valid_strict(6usize, &schema);
    assert_valid_strict(157usize, &schema);
    assert_valid_strict(158usize, &schema);

    // some invalid items
    assert_invalid_strict(4usize, &schema);
    assert_invalid_strict(3usize, &schema);
    assert_invalid_strict(0usize, &schema);
    assert_invalid_strict(159usize, &schema);
    assert_invalid_strict(200usize, &schema);
    assert_invalid_strict(20_000_000_000u64, &schema);
}

#[test]
fn schema2() {
    let schema = single_schema(TUInt::try_new(999u32, std::u64::MAX).unwrap());

    // some valid items
    assert_valid_strict(999usize, &schema);
    assert_valid_strict(1000usize, &schema);
    assert_valid_strict(std::u64::MAX, &schema);

    // some invalid items
    assert_invalid_strict(998usize, &schema);
    assert_invalid_strict(0usize, &schema);
}

#[test]
fn schema_big() {
    let schema = single_schema(TUInt::try_new(1u128, std::u128::MAX - 2).unwrap());

    // some valid items
    assert_valid_strict(1u32, &schema);
    assert_valid_strict(2u32, &schema);
    assert_valid_strict(std::u128::MAX - 2, &schema);
    assert_valid_strict(std::u128::MAX - 3, &schema);

    // some invalid items
    assert_invalid_strict(0u32, &schema);
    assert_invalid_strict(std::u128::MAX - 1, &schema);
    assert_invalid_strict(std::u128::MAX, &schema);
}

#[test]
fn ordering() {
    let schema = TUInt::try_new(0u32, std::u64::MAX).unwrap();

    ord_assert_equal(schema.clone(), 100usize, 100usize);
    ord_assert_equal(schema.clone(), 0usize, 0usize);
    ord_assert_equal(schema.clone(), std::u64::MAX, std::u64::MAX);

    ord_assert_ascending(schema.clone(), 0u64, std::u64::MAX);
    ord_assert_ascending(schema.clone(), 5usize, 6usize);
}
