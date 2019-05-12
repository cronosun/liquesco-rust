use crate::common::error::LqError;
use crate::common::ine_range::I64IneRange;
use crate::schema::core::Context;
use crate::schema::core::Validator;
use crate::serialization::core::DeSerializer;
use crate::serialization::tsint::SInt64;
use std::cmp::Ordering;
use crate::common::range::LqRangeBounds;

#[derive(new, Clone)]
pub struct VSInt {
    pub range: I64IneRange,
}

impl VSInt {
    pub fn try_new(min: i64, max: i64) -> Result<Self, LqError> {
        Result::Ok(VSInt::new(I64IneRange::try_new_msg(
            "Signed integer range",
            min,
            max,
        )?))
    }
}

impl Validator<'static> for VSInt {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let int_value = SInt64::de_serialize(context.reader())?;
        self.range
            .require_within("Signed integer schema validation", &int_value)?;
        Result::Ok(())
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
        let int1 = SInt64::de_serialize(r1)?;
        let int2 = SInt64::de_serialize(r2)?;
        Result::Ok(int1.cmp(&int2))
    }
}
