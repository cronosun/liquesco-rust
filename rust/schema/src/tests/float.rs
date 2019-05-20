use liquesco_common::float::F32Ext;
use liquesco_common::float::F64Ext;
use liquesco_common::range::NewFull;
use liquesco_common::range::Range;
use crate::doc_type::DocType;
use crate::float::TFloat32;
use crate::float::TFloat64;
use crate::tests::ordering::ord_assert_ascending;
use crate::tests::ordering::ord_assert_equal;
use crate::tests::utils::assert_invalid_strict;
use crate::tests::utils::assert_valid_strict;
use crate::tests::utils::single_schema;

#[test]
fn schema1_32() {
    let schema = single_schema(DocType::from(TFloat32::new(Range::<F32Ext>::full())));

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
    let schema = single_schema(DocType::from(TFloat64::new(Range::<F64Ext>::full())));

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
    let mut float = TFloat32::new(
        Range::<F32Ext>::try_inclusive(F32Ext::from(-14.5f32), F32Ext::from(19.7f32)).unwrap(),
    );
    float.allow_nan = true;
    float.allow_positive_infinity = true;
    float.allow_negative_infinity = true;
    let schema = single_schema(DocType::from(float));

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
    let mut float = TFloat64::new(
        Range::<F64Ext>::try_inclusive(F64Ext::from(-14.5f64), F64Ext::from(19.7f64)).unwrap(),
    );
    float.allow_nan = true;
    float.allow_positive_infinity = true;
    float.allow_negative_infinity = true;
    let schema = single_schema(DocType::from(float));

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

#[test]
fn ordering_64() {
    let mut schema = TFloat64::new(
        Range::<F64Ext>::try_inclusive(std::f64::MIN.into(), std::f64::MAX.into()).unwrap(),
    );
    schema.allow_nan = true;
    schema.allow_positive_infinity = true;
    schema.allow_negative_infinity = true;
    let schema = DocType::from(schema);

    // nan is equal to itself
    ord_assert_equal(schema.clone(), std::f64::NAN, std::f64::NAN);
    // infinity is equal to itself
    ord_assert_equal(schema.clone(), std::f64::INFINITY, std::f64::INFINITY);
    ord_assert_equal(
        schema.clone(),
        std::f64::NEG_INFINITY,
        std::f64::NEG_INFINITY,
    );
    // and values of course
    ord_assert_equal(schema.clone(), 1.278f64, 1.278f64);

    // nan is always the smallest thing
    ord_assert_ascending(schema.clone(), std::f64::NAN, -100f64);
    ord_assert_ascending(schema.clone(), std::f64::NAN, std::f64::INFINITY);
    ord_assert_ascending(schema.clone(), std::f64::NAN, std::f64::NEG_INFINITY);

    // except for nan, negative infinity is always the smallest thing
    ord_assert_ascending(schema.clone(), std::f64::NEG_INFINITY, -100f64);
    ord_assert_ascending(schema.clone(), std::f64::NEG_INFINITY, std::f64::MIN);
    ord_assert_ascending(schema.clone(), std::f64::NEG_INFINITY, std::f64::INFINITY);

    // positive infinity is always the largest thing
    ord_assert_ascending(schema.clone(), 1000000f64, std::f64::INFINITY);
    ord_assert_ascending(schema.clone(), std::f64::MAX, std::f64::INFINITY);
    ord_assert_ascending(schema.clone(), std::f64::NEG_INFINITY, std::f64::INFINITY);

    // and normal values
    ord_assert_ascending(schema.clone(), 0.01f64, 0.011f64);
}

#[test]
fn ordering_32() {
    let mut schema = TFloat32::new(
        Range::<F32Ext>::try_inclusive(std::f32::MIN.into(), std::f32::MAX.into()).unwrap(),
    );
    schema.allow_nan = true;
    schema.allow_positive_infinity = true;
    schema.allow_negative_infinity = true;
    let schema = DocType::from(schema);

    // nan is equal to itself
    ord_assert_equal(schema.clone(), std::f32::NAN, std::f32::NAN);
    // infinity is equal to itself
    ord_assert_equal(schema.clone(), std::f32::INFINITY, std::f32::INFINITY);
    ord_assert_equal(
        schema.clone(),
        std::f32::NEG_INFINITY,
        std::f32::NEG_INFINITY,
    );
    // and values of course
    ord_assert_equal(schema.clone(), 1.278f32, 1.278f32);

    // nan is always the smallest thing
    ord_assert_ascending(schema.clone(), std::f32::NAN, -100f32);
    ord_assert_ascending(schema.clone(), std::f32::NAN, std::f32::INFINITY);
    ord_assert_ascending(schema.clone(), std::f32::NAN, std::f32::NEG_INFINITY);

    // except for nan, negative infinity is always the smallest thing
    ord_assert_ascending(schema.clone(), std::f32::NEG_INFINITY, -100f32);
    ord_assert_ascending(schema.clone(), std::f32::NEG_INFINITY, std::f32::MIN);
    ord_assert_ascending(schema.clone(), std::f32::NEG_INFINITY, std::f32::INFINITY);

    // positive infinity is always the largest thing
    ord_assert_ascending(schema.clone(), 1000000f32, std::f32::INFINITY);
    ord_assert_ascending(schema.clone(), std::f32::MAX, std::f32::INFINITY);
    ord_assert_ascending(schema.clone(), std::f32::NEG_INFINITY, std::f32::INFINITY);

    // and normal values
    ord_assert_ascending(schema.clone(), 0.01f32, 0.011f32);
}
