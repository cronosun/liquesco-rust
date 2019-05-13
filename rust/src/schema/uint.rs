use std::cmp::Ordering;
use crate::common::error::LqError;
use crate::common::ine_range::U64IneRange;
use crate::schema::core::Context;
use crate::schema::core::Type;
use crate::serialization::core::DeSerializer;
use crate::serialization::uint::UInt64;
use crate::common::range::LqRangeBounds;

#[derive(new, Clone, Debug)]
pub struct TUInt {
    pub range: U64IneRange,
}

impl TUInt {
    pub fn try_new(min: u64, max: u64) -> Result<Self, LqError> {
        Result::Ok(TUInt::new(U64IneRange::try_new_msg(
            "Unsigned integer range",
            min,
            max,
        )?))
    }
}

impl Type<'static> for TUInt {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let int_value = UInt64::de_serialize(context.reader())?;
        self.range
            .require_within("Unsigned integer schema validation", &int_value)?;
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
        let int1 = UInt64::de_serialize(r1)?;
        let int2 = UInt64::de_serialize(r2)?;
        Result::Ok(int1.cmp(&int2))
    }
}
