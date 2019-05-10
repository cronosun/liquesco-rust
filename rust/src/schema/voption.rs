use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Validator;
use crate::schema::core::ValidatorRef;
use crate::schema::validators::AnyValidator;
use crate::serialization::core::DeSerializer;
use crate::serialization::toption::Presence;

#[derive(new, Clone)]
pub struct VOption {
    pub validator: ValidatorRef,
}

impl<'a> From<VOption> for AnyValidator<'static> {
    fn from(value: VOption) -> Self {
        AnyValidator::Option(value)
    }
}

impl Validator<'static> for VOption {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let presence = Presence::de_serialize(context.reader())?;

        match presence {
            Presence::Absent => Result::Ok(()),
            Presence::Present => context.validate(self.validator),
        }
    }
}
