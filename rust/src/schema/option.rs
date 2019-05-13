use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Type;
use crate::schema::core::TypeRef;
use crate::serialization::core::DeSerializer;
use crate::serialization::option::Presence;
use std::cmp::Ordering;

#[derive(new, Clone, Debug)]
pub struct TOption {
    pub r#type: TypeRef,
}

impl Type<'static> for TOption {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let presence = Presence::de_serialize(context.reader())?;

        match presence {
            Presence::Absent => Result::Ok(()),
            Presence::Present => context.validate(self.r#type),
        }
    }

    fn compare<'c, C>(
        &self,
        context: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>,
    {
        let presence1 = Presence::de_serialize(r1)?;
        let presence2 = Presence::de_serialize(r2)?;

        match (presence1, presence2) {
            (Presence::Absent, Presence::Absent) => Result::Ok(Ordering::Equal),
            (Presence::Present, Presence::Present) => context.compare(self.r#type, r1, r2),
            (Presence::Absent, Presence::Present) => {
                // "absent" < "present"
                Result::Ok(Ordering::Less)
            }
            (Presence::Present, Presence::Absent) => Result::Ok(Ordering::Greater),
        }
    }
}
