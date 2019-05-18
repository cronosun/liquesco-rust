use core::cmp::Ordering;
use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Type;
use crate::serialization::core::DeSerializer;
use crate::serialization::binary::Binary;
use crate::serialization::uuid::Uuid;

#[derive(new, Clone, Debug)]
pub struct TUuid;

impl Type for TUuid {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        // it's just a normal binary
        Uuid::de_serialize(context.reader())?;
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
        // compare like "normal" binaries
        let bin1 = Binary::de_serialize(r1)?;
        let bin2 = Binary::de_serialize(r2)?;
        Result::Ok(bin1.cmp(&bin2))
    }
}
