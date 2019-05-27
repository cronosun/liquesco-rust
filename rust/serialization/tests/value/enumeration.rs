use crate::value::check_value;
use liquesco_serialization::value::EnumValue;
use liquesco_serialization::value::Value;

#[test]
fn no_value_enum() {
    let enum_value = EnumValue::new_no_value(0);
    check_value(&enum_value.into());
    let enum_value = EnumValue::new_no_value(1);
    check_value(&enum_value.into());
    let enum_value = EnumValue::new_no_value(17000);
    check_value(&enum_value.into());
}

#[test]
fn with_value_enum() {
    let value: Value<'static> = "hello".into();
    let enum_value = EnumValue::new(45345233, value);
    check_value(&enum_value.into());
}
