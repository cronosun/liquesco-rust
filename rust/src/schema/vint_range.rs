use crate::common::range::U64IneRange;
use crate::common::error::LqError;
use crate::common::range::I64IneRange;
use crate::schema::core::Context;
use crate::schema::core::Validator;
use crate::schema::validators::AnyValidator;
use crate::serialization::core::DeSerializer;
use crate::serialization::tseq::SeqHeader;
use crate::serialization::tsint::SInt64;
use crate::serialization::tuint::UInt64;

// TODO: Glaube das nehmen wir wieder raus`?

#[derive(new, Clone)]
pub struct VSIntRange {
    pub range: I64IneRange,
}

#[derive(new, Clone)]
pub struct VUIntRange {
    pub range: U64IneRange,
}

impl VSIntRange {
    pub fn try_new(min: i64, max: i64) -> Result<Self, LqError> {
        Result::Ok(VSIntRange::new(I64IneRange::try_new_msg(
            "Range of signed integer range",
            min,
            max,
        )?))
    }
}

impl VUIntRange {
    pub fn try_new(min: u64, max: u64) -> Result<Self, LqError> {
        Result::Ok(VUIntRange::new(U64IneRange::try_new_msg(
            "Range of unsigned integer range",
            min,
            max,
        )?))
    }
}

impl From<VSIntRange> for AnyValidator<'static> {
    fn from(value: VSIntRange) -> Self {
        AnyValidator::SIntRange(value)
    }
}

impl From<VUIntRange> for AnyValidator<'static> {
    fn from(value: VUIntRange) -> Self {
        AnyValidator::UIntRange(value)
    }
}

impl Validator<'static> for VSIntRange {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        // range is in a struct (min/max)
        let seq = SeqHeader::de_serialize(context.reader())?;
        if seq.length() != 2 {
            return LqError::err_new(format!(
                "A range is a sequence \
                 with exacly two elements; got {:?} elements.",
                seq.length()
            ));
        }

        let min = SInt64::de_serialize(context.reader())?;
        let max = SInt64::de_serialize(context.reader())?;

        self.range
            .require_within("Range schema validation (range min value)", &min)?;
        self.range
            .require_within("Range schema validation (range max value)", &max)?;

        // asserts min<=max
        I64IneRange::try_new_msg("Range schema validation", min, max)?;
        Result::Ok(())
    }
}


impl Validator<'static> for VUIntRange {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        // range is in a struct (min/max)
        let seq = SeqHeader::de_serialize(context.reader())?;
        if seq.length() != 2 {
            return LqError::err_new(format!(
                "A range is a sequence \
                 with exacly two elements; got {:?} elements.",
                seq.length()
            ));
        }

        let min = UInt64::de_serialize(context.reader())?;
        let max = UInt64::de_serialize(context.reader())?;

        self.range
            .require_within("Range schema validation (range min value)", &min)?;
        self.range
            .require_within("Range schema validation (range max value)", &max)?;

        // asserts min<=max
        U64IneRange::try_new_msg("Range schema validation", min, max)?;
        Result::Ok(())
    }
}
