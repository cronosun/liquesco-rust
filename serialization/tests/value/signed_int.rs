use crate::value::check_value;

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
    check_value(&(-2_147_483_648).into());
    check_value(&(-2_147_483_647).into());
}

#[test]
fn test_i64() {
    check_value(&(-9_223_372_036_854_775_808i64).into());
    check_value(&(9_223_372_036_854_775_807i64).into());
}

#[test]
fn test_i128() {
    check_value(&(-170141183460469231731687303715884105727i128).into());
    check_value(&(170141183460469231731687303715884105727i128).into());
}
