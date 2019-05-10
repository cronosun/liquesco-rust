use crate::common::error::LqError;
use crate::common::range::U32IneRange;
use crate::schema::core::Context;
use crate::schema::core::Validator;
use crate::schema::core::ValidatorRef;
use crate::schema::validators::AnyValidator;
use crate::serialization::core::DeSerializer;
use crate::serialization::tseq::SeqHeader;

#[derive(new, Clone)]
pub struct VSeq {
    pub element_validator: ValidatorRef,
    pub length: U32IneRange,
}

impl VSeq {
    pub fn try_new(
        element_validator: ValidatorRef,
        min_len: u32,
        max_len: u32,
    ) -> Result<Self, LqError> {
        Result::Ok(Self {
            element_validator,
            length: U32IneRange::try_new(min_len, max_len)?,
        })
    }
}

impl Validator<'static> for VSeq {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let seq = SeqHeader::de_serialize(context.reader())?;
        let number_of_items = seq.length();

        self.length.require_within(
            "Sequence length validation \
             (schema; min/max elements in sequence)",
            &number_of_items,
        )?;

        // validate each element
        for _ in 0..number_of_items {
            context.validate(self.element_validator)?;
        }

        Result::Ok(())
    }
}

impl<'a> From<VSeq> for AnyValidator<'a> {
    fn from(value: VSeq) -> Self {
        AnyValidator::Seq(value)
    }
}
