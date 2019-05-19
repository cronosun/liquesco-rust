use crate::tests::value::utils::check_value;

#[test]
fn test_i8() {
    check_value(&0i64.into());
    check_value(&(-1i64).into());
    check_value(&(-128).into());
}

#[test]
fn test_i16() {
    check_value(&(-3458i64).into());
}

#[test]
fn test_i32() {
    check_value(&(-2147483648).into());
    check_value(&(-2147483647).into());
}

#[test]
fn test_i64() {
    check_value(&(-9223372036854775808i64).into());
    check_value(&(9223372036854775807i64).into());
}
