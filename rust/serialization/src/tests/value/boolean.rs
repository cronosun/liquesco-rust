use crate::tests::value::utils::check_value;

#[test]
fn test_true() {
    check_value(&true.into());
}

#[test]
fn test_false() {
    check_value(&false.into());
}
