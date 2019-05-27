use crate::value::check_value;

#[test]
fn test_u8() {
    check_value(&0u64.into());
    check_value(&1u64.into());
    check_value(&127.into());
    check_value(&255.into());
}

#[test]
fn test_u16() {
    check_value(&3458u64.into());
    check_value(&256u64.into());
    check_value(&65535.into());
}

#[test]
fn test_u32() {
    check_value(&4294967295u32.into());
}

#[test]
fn test_u64() {
    check_value(&18446744073709551615u64.into());
}