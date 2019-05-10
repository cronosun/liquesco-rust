use crate::schema::core::Context;
use crate::common::error::LqError;
use crate::schema::core::Validator;
use crate::schema::validators::AnyValidator;
use crate::serialization::core::DeSerializer;
use crate::serialization::tuint::UInt64;
use crate::common::range::U64IneRange;

#[derive(new, Clone)]
pub struct VUInt {
    pub range : U64IneRange    
}

impl VUInt {
    pub fn try_new(min: u64, max: u64) -> Result<Self, LqError> {
        Result::Ok(VUInt::new(U64IneRange::try_new_msg("Unsigned integer range", min, max)?))
    }
}

impl From<VUInt> for AnyValidator<'static> {
    fn from(value: VUInt) -> Self {
        AnyValidator::UInt(value)
    }
}

impl Validator<'static> for VUInt {

    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c> {
        let int_value = UInt64::de_serialize(context.reader())?;
        self.range.require_within("Unsigned integer schema validation", &int_value)?;
        Result::Ok(())
    }
}
