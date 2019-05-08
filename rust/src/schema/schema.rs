use crate::common::error::LqError;
use crate::schema::core::Config;
use crate::schema::core::Context;
use crate::schema::core::Schema;
use crate::schema::core::ValidatorContainer;
use crate::schema::core::ValidatorRef;
use crate::serialization::core::BinaryReader;
use std::marker::PhantomData;

pub struct DefaultSchema<'a, C: ValidatorContainer<'a>> {
    validators: C,
    main_reference: ValidatorRef,
    _phantom: &'a PhantomData<()>,
}

impl<'a, C: ValidatorContainer<'a>> Schema for DefaultSchema<'a, C> {
    fn validate<'r, R: BinaryReader<'r>>(
        &self,
        config: Config,
        reader: &mut R,
    ) -> Result<(), LqError> {
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
    fn validate_internal<'c, 'r, R: BinaryReader<'r>>(
        &'c self,
        config: Config,
        reader: &'c mut R,
    ) -> Result<(), LqError> {
        let mut context = ValidationContext {
            validators: &self.validators,
            config,
            reader,
            _phantom1: &PhantomData,
            _phantom2: &PhantomData,
        };
        context.validate(self.main_reference)
    }
}

struct ValidationContext<'s, 'c, 'r, C: ValidatorContainer<'c>, R: BinaryReader<'r>> {
    validators: &'s C,
    config: Config,
    reader: &'s mut R,
    _phantom1: &'c PhantomData<()>,
    _phantom2: &'r PhantomData<()>,
}

impl<'s, 'c, 'r, C: ValidatorContainer<'c>, R: BinaryReader<'r>> Context<'r>
    for ValidationContext<'s, 'c, 'r, C, R>
{
    type Reader = R;

    fn validate(&mut self, reference: ValidatorRef) -> Result<(), LqError> {
        if let Some(validator) = self.validators.validators(reference) {
            validator.validate(self)
        } else {
            LqError::err_new(format!(
                "Validator (reference {:?}) not found. \
                 Unable to validate.",
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
}
