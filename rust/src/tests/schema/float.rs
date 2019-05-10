use crate::common::range::F32IneRange;
use crate::common::range::F64IneRange;
use crate::common::range::NewFull;
use crate::schema::vfloat::NumberConstraint;
use crate::schema::vfloat::VFloat32;
use crate::schema::vfloat::VFloat64;
use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;
use crate::tests::schema::utils::single_schema;

#[test]
fn schema1_32() {
    let schema = single_schema(VFloat32::new(NumberConstraint::Range(F32IneRange::full())));

    // some valid items
    assert_valid_strict(-0.0f32, &schema);
    assert_valid_strict(0.0f32, &schema);
    assert_valid_strict(-458.0f32, &schema);
    assert_valid_strict(458.0f32, &schema);
    assert_valid_strict(std::f32::MIN, &schema);
    assert_valid_strict(std::f32::MAX, &schema);
    assert_valid_strict(std::f32::MIN_POSITIVE, &schema);

    // some invalid items
    assert_invalid_strict(std::f32::NAN, &schema);
    assert_invalid_strict(std::f32::NEG_INFINITY, &schema);
    assert_invalid_strict(std::f32::INFINITY, &schema);
}

#[test]
fn schema1_64() {
    let schema = single_schema(VFloat64::new(NumberConstraint::Range(F64IneRange::full())));

    // some valid items
    assert_valid_strict(-0.0f64, &schema);
    assert_valid_strict(0.0f64, &schema);
    assert_valid_strict(-458.0f64, &schema);
    assert_valid_strict(458.0f64, &schema);
    assert_valid_strict(std::f64::MIN, &schema);
    assert_valid_strict(std::f64::MAX, &schema);
    assert_valid_strict(std::f64::MIN_POSITIVE, &schema);

    // some invalid items
    assert_invalid_strict(std::f64::NAN, &schema);
    assert_invalid_strict(std::f64::NEG_INFINITY, &schema);
    assert_invalid_strict(std::f64::INFINITY, &schema);
}

#[test]
fn schema2_32() {
    let mut float = VFloat32::new(NumberConstraint::Range(
        F32IneRange::try_new(-14.5f32, 19.7f32).unwrap(),
    ));
    float.allow_nan = true;
    float.allow_positive_infinity = true;
    float.allow_negative_infinity = true;
    let schema = single_schema(float);

    // some valid items
    assert_valid_strict(-14.5f32, &schema);
    assert_valid_strict(19.7f32, &schema);
    assert_valid_strict(-14.49f32, &schema);
    assert_valid_strict(19.69f32, &schema);
    assert_valid_strict(std::f32::NAN, &schema);
    assert_valid_strict(std::f32::NEG_INFINITY, &schema);
    assert_valid_strict(std::f32::INFINITY, &schema);

    // some invalid items
    assert_invalid_strict(-14.51f32, &schema);
    assert_invalid_strict(19.71f32, &schema);
}

#[test]
fn schema2_64() {
    let mut float = VFloat64::new(NumberConstraint::Range(
        F64IneRange::try_new(-14.5f64, 19.7f64).unwrap(),
    ));
    float.allow_nan = true;
    float.allow_positive_infinity = true;
    float.allow_negative_infinity = true;
    let schema = single_schema(float);

    // some valid items
    assert_valid_strict(-14.5f64, &schema);
    assert_valid_strict(19.7f64, &schema);
    assert_valid_strict(-14.49f64, &schema);
    assert_valid_strict(19.69f64, &schema);
    assert_valid_strict(std::f64::NAN, &schema);
    assert_valid_strict(std::f64::NEG_INFINITY, &schema);
    assert_valid_strict(std::f64::INFINITY, &schema);

    // some invalid items
    assert_invalid_strict(-14.51f64, &schema);
    assert_invalid_strict(19.71f64, &schema);
}