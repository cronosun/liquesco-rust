use crate::schema::vsint::VSInt;
use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;
use crate::tests::schema::utils::single_schema;

#[test]
fn schema1() {
    let schema = single_schema(VSInt::try_new(-45, 4443444).unwrap());

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
