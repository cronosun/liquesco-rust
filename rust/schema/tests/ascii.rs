mod common;

use liquesco_schema::ascii::TAscii;
use liquesco_schema::doc_type::DocType;
use common::ordering::ord_assert_ascending;
use common::ordering::ord_assert_equal;
use common::utils::assert_invalid_strict;
use common::utils::assert_valid_strict;
use common::utils::single_schema;

#[test]
fn schema1() {
    let schema = single_schema(DocType::from(TAscii::try_new(5, 15, 97, 122).unwrap()));

    // some valid items
    assert_valid_strict("hello".to_string(), &schema);
    assert_valid_strict("computer".to_string(), &schema);
    assert_valid_strict("under".to_string(), &schema);
    assert_valid_strict("qwertzuiopasdfg".to_string(), &schema);

    // some invalid items
    assert_invalid_strict("hell".to_string(), &schema);
    assert_invalid_strict("qwertzuiopasdfgh".to_string(), &schema);

    assert_invalid_strict("hell`".to_string(), &schema);
    assert_invalid_strict("hell{".to_string(), &schema);
    assert_invalid_strict("NoUppercase".to_string(), &schema);
}

#[test]
fn ordering() {
    let schema = DocType::from(TAscii::try_new(0, 500, 0, 127).unwrap());

    ord_assert_equal(schema.clone(), "".to_string(), "".to_string());
    ord_assert_equal(schema.clone(), "hello".to_string(), "hello".to_string());

    ord_assert_ascending(schema.clone(), "".to_string(), "a".to_string());
    ord_assert_ascending(schema.clone(), "aaaaaaaaaa".to_string(), "ab".to_string());
    ord_assert_ascending(
        schema.clone(),
        "aaaaaaaaaa".to_string(),
        "aaaaaaaaaab".to_string(),
    );
}
