use crate::schema::core::Context;
use crate::common::error::LqError;
use crate::schema::core::Validator;
use crate::schema::validators::Validators;
use crate::serialization::core::DeSerializer;
use crate::serialization::tsint::SInt64;

#[derive(Clone)]
pub struct VSInt {
    min: i64,
    max: i64,
}

impl VSInt {
    pub fn try_new(min: i64, max: i64) -> Result<Self, LqError> {
        if min > max {
            LqError::err_new(format!(
                "Min value ({:?}) is greater then max value ({:?}).",
                min, max
            ))
        } else {
            Result::Ok(Self {
                min,
                max,
            })
        }
    }
}

impl<'a> From<VSInt> for Validators<'a> {
    fn from(value: VSInt) -> Self {
        Validators::SInt(value)
    }
}

impl<'a> Validator<'a> for VSInt {

    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c> {
        let int_value = SInt64::de_serialize(context.reader())?;
        if int_value < self.min {
            return LqError::err_new(format!(
                "Given integer {:?} is too small (minimum \
                 allowed is {:?})",
                int_value, self.min
            ));
        }
        if int_value > self.max {
            return LqError::err_new(format!(
                "Given integer {:?} is too large (maximum \
                 allowed is {:?})",
                int_value, self.max
            ));
        }
        Result::Ok(())
    }
}
