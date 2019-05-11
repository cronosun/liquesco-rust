use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Schema;
use crate::schema::core::ValidatorContainer;
use crate::schema::core::ValidatorRef;
use crate::schema::core::{Config, Validator};
use crate::serialization::core::LqReader;
use std::cmp::Ordering;
use std::marker::PhantomData;

pub struct DefaultSchema<'a, C: ValidatorContainer<'a>> {
    validators: C,
    main_reference: ValidatorRef,
    _phantom: &'a PhantomData<()>,
}

impl<'a, C: ValidatorContainer<'a>> Schema for DefaultSchema<'a, C> {
    fn validate<'r, R: LqReader<'r>>(&self, config: Config, reader: &mut R) -> Result<(), LqError> {
        self.validate_internal(config, reader)
    }
}

impl<'a, C: ValidatorContainer<'a>> DefaultSchema<'a, C> {
    pub fn new(container: C, main_reference: ValidatorRef) -> Self {
        Self {
            validators: container,
            main_reference,
            _phantom: &PhantomData,
        }
    }

    #[inline]
    fn validate_internal<'c, 'r, R: LqReader<'r>>(
        &'c self,
        config: Config,
        reader: &'c mut R,
    ) -> Result<(), LqError> {
        let mut context = ValidationContext {
            validators: &self.validators,
            config,
            reader,
            anchor_index: Option::None,
            max_used_anchor_index: Option::None,
            _phantom1: &PhantomData,
            _phantom2: &PhantomData,
        };
        context.validate(self.main_reference)
    }
}

struct ValidationContext<'s, 'c, 'r, C: ValidatorContainer<'c>, R: LqReader<'r>> {
    validators: &'s C,
    config: Config,
    reader: &'s mut R,
    anchor_index: Option<u32>,
    max_used_anchor_index: Option<u32>,
    _phantom1: &'c PhantomData<()>,
    _phantom2: &'r PhantomData<()>,
}

impl<'s, 'c, 'r, C: ValidatorContainer<'c>, R: LqReader<'r>> Context<'r>
    for ValidationContext<'s, 'c, 'r, C, R>
{
    type Reader = R;

    fn validate(&mut self, reference: ValidatorRef) -> Result<(), LqError> {
        if let Some(validator) = self.validators.validator(reference) {
            validator.validate(self)
        } else {
            LqError::err_new(format!(
                "Validator (reference {:?}) not found. \
                 Unable to validate.",
                reference
            ))
        }
    }

    fn compare(
        &self,
        reference: ValidatorRef,
        r1: &mut Self::Reader,
        r2: &mut Self::Reader,
    ) -> Result<Ordering, LqError> {
        if let Some(validator) = self.validators.validator(reference) {
            validator.compare(self, r1, r2)
        } else {
            LqError::err_new(format!(
                "Validator (reference {:?}) not found. \
                 Unable to validate (compare).",
                reference
            ))
        }
    }

    fn config(&self) -> &Config {
        &self.config
    }

    fn reader(&mut self) -> &mut Self::Reader {
        &mut self.reader
    }

    fn anchor_index(&self) -> Option<u32> {
        self.anchor_index
    }

    fn set_anchor_index(&mut self, value: Option<u32>) {
        self.anchor_index = value
    }

    fn max_used_anchor_index(&self) -> Option<u32> {
        self.max_used_anchor_index
    }

    fn set_max_used_anchor_index(&mut self, value: Option<u32>) {
        self.max_used_anchor_index = value;
    }
}
