use crate::value::Value;
use crate::tests::value::utils::check_value;

#[test]
fn test_none() {
    let option: Option<Value<'static>> = Option::None;
    check_value(&option.into());
}

#[test]
fn test_some() {
    let option: Option<Value<'static>> = Option::Some("hello".into());
    check_value(&option.into());
}
