use crate::doc_type::DocType;
use crate::sint::TSInt;
use crate::tests::ordering::ord_assert_ascending;
use crate::tests::ordering::ord_assert_equal;
use crate::tests::utils::assert_invalid_strict;
use crate::tests::utils::assert_valid_strict;
use crate::tests::utils::single_schema;

#[test]
fn schema1() {
    let schema = single_schema(DocType::from(TSInt::try_new(-45, 4443444).unwrap()));

    // some valid items
    assert_valid_strict(-45isize, &schema);
    assert_valid_strict(5isize, &schema);
    assert_valid_strict(4443444isize, &schema);
    assert_valid_strict(4443443isize, &schema);
    assert_valid_strict(0isize, &schema);

    // some invalid items
    assert_invalid_strict(-46isize, &schema);
    assert_invalid_strict(4443445isize, &schema);
    assert_invalid_strict(std::i64::MIN, &schema);
    assert_invalid_strict(std::i64::MAX, &schema);
}

#[test]
fn ordering() {
    let schema = DocType::from(TSInt::try_new(std::i64::MIN, std::i64::MAX).unwrap());

    ord_assert_equal(schema.clone(), -100isize, -100isize);
    ord_assert_equal(schema.clone(), 0isize, 0isize);
    ord_assert_equal(schema.clone(), std::i64::MAX, std::i64::MAX);

    ord_assert_ascending(schema.clone(), 0i64, std::i64::MAX);
    ord_assert_ascending(schema.clone(), -5isize, 6isize);
}
