use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Validator;
use crate::schema::core::ValidatorRef;
use crate::schema::validators::AnyValidator;
use crate::serialization::core::DeSerializer;
use crate::serialization::tseq::SeqHeader;

#[derive(Clone)]
pub struct VSeq {
    element_validator: ValidatorRef,
    min_len: u32,
    max_len: u32,
}

impl VSeq {
    pub fn try_new(
        element_validator: ValidatorRef,
        min_len: u32,
        max_len: u32,
    ) -> Result<Self, LqError> {
        if min_len > max_len {
            return LqError::err_new(format!(
                "Minimum length {:?} for sequence is > \
                 maximum length {:?}.",
                min_len, max_len
            ));
        }
        Result::Ok(Self {
            element_validator,
            min_len,
            max_len,
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

        if number_of_items > self.max_len {
            return LqError::err_new(format!(
                "There are too many elements in this sequence. Have {:?} elements \
                 - maximum allowed is {:?} elements.",
                number_of_items, self.max_len
            ));
        } else if number_of_items < self.min_len {
            return LqError::err_new(format!(
                "There are not enough elements in this sequence. Have {:?} elements \
                 - minimum required are {:?} elements.",
                number_of_items, self.min_len
            ));
        }

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
