use crate::schema::unicode::LengthType;
use crate::schema::unicode::TUnicode;
use crate::tests::schema::ordering::ord_assert_ascending;
use crate::tests::schema::ordering::ord_assert_equal;
use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;
use crate::tests::schema::utils::single_schema;

#[test]
fn schema1() {
    let schema = single_schema(TUnicode::try_new(5, 10, LengthType::Byte).unwrap());

    // some valid items
    assert_valid_strict("12345".to_string(), &schema);
    assert_valid_strict("1234567890".to_string(), &schema);
    assert_valid_strict("foo bar".to_string(), &schema);
    assert_valid_strict("àà£Éâ".to_string(), &schema); // 10 bytes
    assert_valid_strict("ဪဪဪa".to_string(), &schema); // 10 bytes
    assert_valid_strict("ဪab".to_string(), &schema); // 5 bytes

    // some invalid items
    // too long
    assert_invalid_strict("ဪဪဪab".to_string(), &schema);
    // too short
    assert_invalid_strict("1234".to_string(), &schema);
}

#[test]
fn schema2() {
    let schema = single_schema(TUnicode::try_new(5, 10, LengthType::Utf8Byte).unwrap());

    // some valid items
    assert_valid_strict("12345".to_string(), &schema);
    assert_valid_strict("1234567890".to_string(), &schema);
    assert_valid_strict("ဪဪဪa".to_string(), &schema);
    assert_valid_strict("ဪab".to_string(), &schema);

    // some invalid items
    // too long
    assert_invalid_strict("ဪဪဪab".to_string(), &schema);
    // too short
    assert_invalid_strict("1234".to_string(), &schema);
}

#[test]
fn schema3() {
    let schema = single_schema(TUnicode::try_new(5, 10, LengthType::ScalarValue).unwrap());

    // some valid items
    assert_valid_strict("12345".to_string(), &schema);
    assert_valid_strict("1234567890".to_string(), &schema);
    // 5 scalars
    assert_valid_strict("ဪဪဪab".to_string(), &schema);
    // 10 scalars
    assert_valid_strict("﷽﷽﷽﷽﷽﷽﷽﷽﷽﷽".to_string(), &schema);

    // invalid
    // 11 scalars
    assert_invalid_strict("﷽﷽﷽﷽ဪ﷽﷽﷽﷽﷽ဪ".to_string(), &schema);
    // 4 scalars
    assert_invalid_strict("﷽﷽ဪ".to_string(), &schema);
}

#[test]
fn ordering() {
    let schema = TUnicode::try_new(0, 100, LengthType::ScalarValue).unwrap();

    ord_assert_equal(schema.clone(), "".to_string(), "".to_string());
    ord_assert_equal(schema.clone(), "hello".to_string(), "hello".to_string());
    ord_assert_equal(schema.clone(), "﷽ဪ".to_string(), "﷽ဪ".to_string());

    ord_assert_ascending(schema.clone(), "".to_string(), "a".to_string());
    ord_assert_ascending(schema.clone(), "aaaaaaaaaa".to_string(), "ab".to_string());
    ord_assert_ascending(
        schema.clone(),
        "aaaaaaaaaa".to_string(),
        "aaaaaaaaaab".to_string(),
    );
    ord_assert_ascending(schema.clone(), "aa".to_string(), "a﷽".to_string());
}
