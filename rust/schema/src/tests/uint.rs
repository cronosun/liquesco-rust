use crate::doc_type::DocType;
use crate::uint::TUInt;
use crate::tests::ordering::ord_assert_ascending;
use crate::tests::ordering::ord_assert_equal;
use crate::tests::utils::assert_invalid_strict;
use crate::tests::utils::assert_valid_strict;
use crate::tests::utils::single_schema;

#[test]
fn schema1() {
    let schema = single_schema(DocType::from(TUInt::try_new(5, 158).unwrap()));

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
    assert_invalid_strict(20000000000u64, &schema);
}

#[test]
fn schema2() {
    let schema = single_schema(DocType::from(TUInt::try_new(999, std::u64::MAX).unwrap()));

    // some valid items
    assert_valid_strict(999usize, &schema);
    assert_valid_strict(1000usize, &schema);
    assert_valid_strict(std::u64::MAX, &schema);

    // some invalid items
    assert_invalid_strict(998usize, &schema);
    assert_invalid_strict(0usize, &schema);
}

#[test]
fn ordering() {
    let schema = DocType::from(TUInt::try_new(0, std::u64::MAX).unwrap());

    ord_assert_equal(schema.clone(), 100usize, 100usize);
    ord_assert_equal(schema.clone(), 0usize, 0usize);
    ord_assert_equal(schema.clone(), std::u64::MAX, std::u64::MAX);

    ord_assert_ascending(schema.clone(), 0u64, std::u64::MAX);
    ord_assert_ascending(schema.clone(), 5usize, 6usize);
}
