use crate::schema::core::Context;
use crate::common::error::LqError;
use crate::schema::core::Validator;
use crate::schema::validators::AnyValidator;
use crate::serialization::core::DeSerializer;
use crate::serialization::tuint::UInt64;

#[derive(Clone)]
pub struct VUInt {
    min: u64,
    max: u64,
}

impl VUInt {
    pub fn try_new(min: u64, max: u64) -> Result<Self, LqError> {
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

impl<'a> From<VUInt> for AnyValidator<'a> {
    fn from(value: VUInt) -> Self {
        AnyValidator::UInt(value)
    }
}

impl<'a> Validator<'a> for VUInt {

    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>{
        let int_value = UInt64::de_serialize(context.reader())?;
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
