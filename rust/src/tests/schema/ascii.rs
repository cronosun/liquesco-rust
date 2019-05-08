use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;
use crate::tests::schema::utils::single_schema;
use crate::schema::vascii::VAscii;

#[test]
fn schema1() {
    let schema = single_schema(VAscii::try_new(5, 15, 97, 122).unwrap());
    
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
