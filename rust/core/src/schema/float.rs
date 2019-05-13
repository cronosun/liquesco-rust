use crate::common::range::Range;
use crate::common::error::LqError;
use crate::schema::core::{Context, Type};
use crate::serialization::core::DeSerializer;
use crate::serialization::float::Float32;
use crate::serialization::float::Float64;
use std::cmp::Ordering;
use std::fmt::Debug;
use crate::common::range::LqRangeBounds;

pub type TFloat32 = TFloat<f32>;
pub type TFloat64 = TFloat<f64>;

const NOT_A_NUMBER_ERR_STR: &str = "Expected a float value that is a number. \
                                    This value is not a number (float NaN).";
const NO_POSITIVE_INFINITY: &str = "Positive infinity is not allowed for \
                                    this float value according to the schema.";
const NO_NEGATIVE_INFINITY: &str = "Negative infinity is not allowed for \
                                    this float value according to the schema.";

#[derive(new, Clone, Debug)]
pub struct TFloat<F: PartialOrd + Debug> {
    pub range: Range<F>,
    #[new(value = "false")]
    pub allow_nan: bool,
    #[new(value = "false")]
    pub allow_positive_infinity: bool,
    #[new(value = "false")]
    pub allow_negative_infinity: bool,
}

impl<F: PartialOrd + Debug> TFloat<F> {
    fn validate(
        &self,
        value: F,
        is_nan: bool,
        is_positive_infinity: bool,
        is_negative_infinity: bool,
    ) -> Result<(), LqError> {
        if is_nan {
            // validation for not-a-number
            if !self.allow_nan {
                return LqError::err_static(NOT_A_NUMBER_ERR_STR);
            }
            Result::Ok(())
        } else if is_positive_infinity {
            if !self.allow_positive_infinity {
                return LqError::err_static(NO_POSITIVE_INFINITY);
            }
            Result::Ok(())
        } else if is_negative_infinity {
            if !self.allow_positive_infinity {
                return LqError::err_static(NO_NEGATIVE_INFINITY);
            }
            Result::Ok(())
        } else {
            // it's a number
            self.range.require_within(
                "Float range validation \
                 (schema)",
                &value,
            )
        }
    }
}

impl Type<'static> for TFloat32 {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let float_value = Float32::de_serialize(context.reader())?;
        let (is_nan, is_p_infinite, is_n_infinite) = if float_value.is_nan() {
            (true, false, false)
        } else {
            if float_value.is_infinite() {
                let negative = float_value.is_sign_negative();
                (false, !negative, negative)
            } else {
                (false, false, false)
            }
        };

        self.validate(float_value, is_nan, is_p_infinite, is_n_infinite)
    }

    fn compare<'c, C>(
        &self,
        _: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>,
    {
        let float1 = Float32::de_serialize(r1)?;
        let float2 = Float32::de_serialize(r2)?;

        Result::Ok(cmp_32(float1, float2))
    }
}

impl Type<'static> for TFloat64 {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let float_value = Float64::de_serialize(context.reader())?;
        let (is_nan, is_p_infinite, is_n_infinite) = if float_value.is_nan() {
            (true, false, false)
        } else {
            if float_value.is_infinite() {
                let negative = float_value.is_sign_negative();
                (false, !negative, negative)
            } else {
                (false, false, false)
            }
        };

        self.validate(float_value, is_nan, is_p_infinite, is_n_infinite)
    }

    fn compare<'c, C>(
        &self,
        _: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>,
    {
        let float1 = Float64::de_serialize(r1)?;
        let float2 = Float64::de_serialize(r2)?;

        Result::Ok(cmp_64(float1, float2))
    }
}

/// Unfortunately we MUST have ord for the floats (need something to make sure there is unique ordering in lists)
///
/// Rules:
/// NaN = NaN
/// NaN < Infinite
/// NaN < Number
/// -Infinite < Number < +Infinite
fn cmp_32(v1: f32, v2: f32) -> Ordering {
    if let Some(ord) = v1.partial_cmp(&v2) {
        ord
    } else {
        if v1.is_nan() {
            if v2.is_nan() {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else {
            panic!("Incomplete cmp implementation for float")
        }
    }
}

fn cmp_64(v1: f64, v2: f64) -> Ordering {
    if let Some(ord) = v1.partial_cmp(&v2) {
        ord
    } else {
        if v1.is_nan() {
            if v2.is_nan() {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else {
            panic!("Incomplete cmp implementation for float")
        }
    }
}
