use crate::value::check_value;
use crate::value::serialize_de_serialize;
use liquesco_serialization::float::Float;
use liquesco_serialization::value::Value;
use std::convert::TryFrom;

#[test]
fn test_some_floats_f32() {
    check_value(&Float::from(0.758f32).into());
    check_value(&Float::from(std::f32::MAX).into());
    check_value(&Float::from(std::f32::MIN).into());
    check_value(&Float::from(std::f32::INFINITY).into());
    check_value(&Float::from(0f32).into());

    // special check for NaN
    serialize_de_serialize(&Float::from(std::f32::NAN).into(), |result| match result {
        Value::Float(float) => {
            let v_float: f32 = f32::try_from(float).unwrap();
            assert!(v_float.is_nan());
        }
        _ => panic!("Expected a float!"),
    });
}

#[test]
fn test_some_floats_f64() {
    check_value(&Float::from(0.758f64).into());
    check_value(&Float::from(std::f64::MAX).into());
    check_value(&Float::from(std::f64::MIN).into());
    check_value(&Float::from(std::f64::INFINITY).into());
    check_value(&Float::from(0f64).into());

    // special check for NaN
    serialize_de_serialize(&Float::from(std::f64::NAN).into(), |result| match result {
        Value::Float(float) => {
            let v_float: f64 = f64::try_from(float).unwrap();
            assert!(v_float.is_nan());
        }
        _ => panic!("Expected a float!"),
    });
}
