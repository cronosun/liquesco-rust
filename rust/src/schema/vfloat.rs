use crate::common::error::LqError;
use crate::common::range::IneRange;
use crate::schema::core::{Context, Validator};
use crate::schema::validators::AnyValidator;
use crate::serialization::core::DeSerializer;
use crate::serialization::tfloat::Float32;
use crate::serialization::tfloat::Float64;
use std::fmt::Debug;

pub type VFloat32 = VFloat<f32>;
pub type VFloat64 = VFloat<f64>;

const NOT_A_NUMBER_ERR_STR: &str = "Expected a float value that is a number. \
                                    This value is not a number (float NaN).";
const NO_POSITIVE_INFINITY: &str = "Positive infinity is not allowed for \
                                    this float value according to the schema.";
const NO_NEGATIVE_INFINITY: &str = "Negative infinity is not allowed for \
                                    this float value according to the schema.";

#[derive(new, Clone)]
pub struct VFloat<F: PartialOrd + Debug> {
    pub constraint: NumberConstraint<F>,
    #[new(value = "false")]
    pub allow_nan: bool,
    #[new(value = "false")]
    pub allow_positive_infinity: bool,
    #[new(value = "false")]
    pub allow_negative_infinity: bool,
}

#[derive(new, Clone)]
pub enum NumberConstraint<F: PartialOrd + Debug> {
    NoNumbers,
    Range(IneRange<F>),
}

impl From<VFloat32> for AnyValidator<'static> {
    fn from(value: VFloat32) -> Self {
        AnyValidator::Float32(value)
    }
}

impl From<VFloat64> for AnyValidator<'static> {
    fn from(value: VFloat64) -> Self {
        AnyValidator::Float64(value)
    }
}

impl<F: PartialOrd + Debug> NumberConstraint<F> {
    fn validate_given_number(&self, number: F) -> Result<(), LqError> {
        match self {
            NumberConstraint::NoNumbers => LqError::err_new(format!(
                "According to the schema no numbers \
                 are allowed (maybe only NaN; positive and/or negative infinity). \
                 Got number {:?}.",
                number
            )),
            NumberConstraint::Range(range) => {
                if !range.contains(&number) {
                    LqError::err_new(format!(
                        "According to the schema only \
                         numbers in the range {:?} are allowed. Got number {:?}.",
                        range, number
                    ))
                } else {
                    Result::Ok(())
                }
            }
        }
    }
}

impl<F: PartialOrd + Debug> VFloat<F> {
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
            self.constraint.validate_given_number(value)
        }
    }
}

impl Validator<'static> for VFloat32 {
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
}


impl Validator<'static> for VFloat64 {
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
}
