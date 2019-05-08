use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;
use crate::tests::schema::utils::single_schema;
use crate::schema::vuint::VUInt;

#[test]
fn schema1() {
    let schema = single_schema(VUInt::try_new(5, 158).unwrap());
    
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
    let schema = single_schema(VUInt::try_new(999, std::u64::MAX).unwrap());
    
    // some valid items
    assert_valid_strict(999usize, &schema);    
    assert_valid_strict(1000usize, &schema);    
    assert_valid_strict(std::u64::MAX, &schema);    

    // some invalid items
    assert_invalid_strict(998usize, &schema);
    assert_invalid_strict(0usize, &schema);
}