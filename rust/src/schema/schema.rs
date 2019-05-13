use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Schema;
use crate::schema::core::TypeContainer;
use crate::schema::core::TypeRef;
use crate::schema::core::{Config, Type};
use crate::serialization::core::LqReader;
use std::cmp::Ordering;
use std::marker::PhantomData;

pub struct DefaultSchema<'a, C: TypeContainer<'a>> {
    types: C,
    main_reference: TypeRef,
    _phantom: &'a PhantomData<()>,
}

impl<'a, C: TypeContainer<'a>> Schema for DefaultSchema<'a, C> {
    fn validate<'r, R: LqReader<'r>>(&self, config: Config, reader: &mut R) -> Result<(), LqError> {
        self.validate_internal(config, reader)
    }
}

impl<'a, C: TypeContainer<'a>> DefaultSchema<'a, C> {
    pub fn new(types: C, main_reference: TypeRef) -> Self {
        Self {
            types,
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
            types: &self.types,
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

struct ValidationContext<'s, 'c, 'r, C: TypeContainer<'c>, R: LqReader<'r>> {
    types: &'s C,
    config: Config,
    reader: &'s mut R,
    anchor_index: Option<u32>,
    max_used_anchor_index: Option<u32>,
    _phantom1: &'c PhantomData<()>,
    _phantom2: &'r PhantomData<()>,
}

impl<'s, 'c, 'r, C: TypeContainer<'c>, R: LqReader<'r>> Context<'r>
    for ValidationContext<'s, 'c, 'r, C, R>
{
    type Reader = R;

    fn validate(&mut self, reference: TypeRef) -> Result<(), LqError> {
        if let Some(any_type) = self.types.maybe_type(reference) {
            any_type.validate(self)
        } else {
            LqError::err_new(format!(
                "Type (reference {:?}) not found. \
                 Unable to validate.",
                reference
            ))
        }
    }

    fn compare(
        &self,
        reference: TypeRef,
        r1: &mut Self::Reader,
        r2: &mut Self::Reader,
    ) -> Result<Ordering, LqError> {
        if let Some(any_type) = self.types.maybe_type(reference) {
            any_type.compare(self, r1, r2)
        } else {
            LqError::err_new(format!(
                "Type (reference {:?}) not found. \
                 Unable to validate (compare).",
                reference
            ))
        }
    }

    fn reader(&mut self) -> &mut Self::Reader {
        &mut self.reader
    }

    fn config(&self) -> &Config {
        &self.config
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
