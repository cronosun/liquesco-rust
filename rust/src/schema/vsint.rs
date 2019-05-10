use crate::common::range::I64IneRange;
use crate::schema::core::Context;
use crate::common::error::LqError;
use crate::schema::core::Validator;
use crate::schema::validators::AnyValidator;
use crate::serialization::core::DeSerializer;
use crate::serialization::tsint::SInt64;

#[derive(new, Clone)]
pub struct VSInt {
    pub range : I64IneRange    
}

impl VSInt {
    pub fn try_new(min: i64, max: i64) -> Result<Self, LqError> {
        Result::Ok(VSInt::new(I64IneRange::try_new_msg("Signed integer range", min, max)?))
    }
}

impl From<VSInt> for AnyValidator<'static> {
    fn from(value: VSInt) -> Self {
        AnyValidator::SInt(value)
    }
}

impl Validator<'static> for VSInt {

    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c> {
        let int_value = SInt64::de_serialize(context.reader())?;
        self.range.require_within("Signed integer schema validation", &int_value)?;
        Result::Ok(())
    }
}
