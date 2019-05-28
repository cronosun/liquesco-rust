use crate::any_type::AnyType;
use crate::core::Context;
use crate::core::Schema;
use crate::core::TypeContainer;
use crate::core::TypeRef;
use crate::core::{Config, Type};
use liquesco_common::error::LqError;
use liquesco_serialization::core::LqReader;
use std::cmp::Ordering;
use std::marker::PhantomData;
use liquesco_serialization::value::Value;
use liquesco_serialization::core::DeSerializer;

pub struct DefaultSchema<'a, C: TypeContainer<'a>> {
    types: C,
    main_reference: TypeRef,
    extended_diagnostics : bool,
    _phantom: &'a PhantomData<()>,
}

impl<'a, C: TypeContainer<'a>> Schema<'a> for DefaultSchema<'a, C> {
    fn validate<'r, R: LqReader<'r>>(&self, config: Config, reader: &mut R) -> Result<(), LqError> {
        self.validate_internal(config, reader)
    }

    fn main_type(&self) -> TypeRef {
        self.main_reference
    }
}

impl<'a, C: TypeContainer<'a>> TypeContainer<'a> for DefaultSchema<'a, C> {
    fn maybe_type(&self, reference: TypeRef) -> Option<&AnyType<'a>> {
        self.types.maybe_type(reference)
    }
}

impl<'a, C: TypeContainer<'a>> DefaultSchema<'a, C> {
    pub fn new(types: C, main_reference: TypeRef) -> Self {
        Self {
            types,
            main_reference,
            extended_diagnostics : false,
            _phantom: &PhantomData,
        }
    }

    pub fn set_extended_diagnostics(&mut self, enabled : bool) {
        self.extended_diagnostics = enabled;
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
            extended_diagnostics : self.extended_diagnostics,
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
    extended_diagnostics : bool,
    _phantom1: &'c PhantomData<()>,
    _phantom2: &'r PhantomData<()>,
}

impl<'s, 'c, 'r, C: TypeContainer<'c>, R: LqReader<'r>> Context<'r>
    for ValidationContext<'s, 'c, 'r, C, R>
{
    type Reader = R;

    fn validate(&mut self, reference: TypeRef) -> Result<(), LqError> {
        if let Some(any_type) = self.types.maybe_type(reference) {
            if self.extended_diagnostics {
                let saved_reader = self.reader.clone();
                let result = any_type.validate(self);
                if let Err(err) = result {
                    Err(enrich_validation_error(err, saved_reader, any_type))
                } else {
                    Ok(())
                }
            } else {
                any_type.validate(self)
            }
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

fn enrich_validation_error<'a, R: LqReader<'a>>(
    err : LqError, mut reader : R, r#type : &AnyType) -> LqError {
    let mut reader_content_description = reader.clone();
    let content_description =
        reader_content_description.read_content_description();

    let mut reader_next_10_bytes = reader.clone();
    let next_20_bytes = reader_next_10_bytes.read_slice(10);

    let value = Value::de_serialize(&mut reader);
    let value_str = match value {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{:?}", err)
    };

    let new_msg = format!("{}. Extended diagnostics:\n\n - Type to validate: {:?}\n\n - \
    Content description: {:?}\n\n - 10 bytes: {:?}\n\n - Value: {}",
        err.msg(), r#type, content_description, next_20_bytes, value_str);
    err.with_msg(new_msg)
}
