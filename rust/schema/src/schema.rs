use crate::any_type::AnyType;
use crate::context::Context;
use crate::context::KeyRefInfo;
use crate::core::Schema;
use crate::core::TypeContainer;
use crate::core::TypeRef;
use crate::core::{Config, Type};
use crate::identifier::Identifier;
use crate::metadata::MetadataSetter;
use crate::root_map::TRootMap;
use crate::schema_builder::{BuildsOwnSchema, SchemaBuilder};
use liquesco_common::error::LqError;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::core::LqReader;
use liquesco_serialization::value::Value;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::marker::PhantomData;

/// Builds the liquesco schema schema.
pub fn schema_schema<B>(mut builder: B) -> Result<B::TTypeContainer, LqError>
where
    B: SchemaBuilder<'static>,
{
    let any_type = AnyType::build_schema(&mut builder);
    let identifier = Identifier::build_schema(&mut builder);

    builder.finish(
        TRootMap::new(any_type.clone(), identifier, any_type).with_doc("The liquesco schema."),
    )
}

pub struct DefaultSchema<'a, C: TypeContainer<'a>> {
    types: C,
    extended_diagnostics: bool,
    _phantom: &'a PhantomData<()>,
}

impl<'a, C: TypeContainer<'a>> Schema<'a> for DefaultSchema<'a, C> {
    fn validate<'r, R: LqReader<'r>>(&self, config: Config, reader: &mut R) -> Result<(), LqError> {
        self.validate_internal(config, reader)
    }
}

impl<'a, C: TypeContainer<'a>> TypeContainer<'a> for DefaultSchema<'a, C> {
    fn maybe_type(&self, reference: &TypeRef) -> Option<&AnyType<'a>> {
        self.types.maybe_type(reference)
    }

    fn root(&self) -> &AnyType<'a> {
        self.types.root()
    }

    fn identifier(&self, reference: &TypeRef) -> Option<Cow<Identifier>> {
        self.types.identifier(reference)
    }

    fn require_type(&self, reference: &TypeRef) -> Result<&AnyType<'a>, LqError> {
        self.types.require_type(reference)
    }
}

impl<'a, T: TypeContainer<'a>> From<T> for DefaultSchema<'a, T> {
    fn from(container: T) -> Self {
        Self::new(container)
    }
}

impl<'a, C: TypeContainer<'a>> DefaultSchema<'a, C> {
    pub fn new(types: C) -> Self {
        Self {
            types,
            extended_diagnostics: false,
            _phantom: &PhantomData,
        }
    }

    pub fn set_extended_diagnostics(&mut self, enabled: bool) {
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
            extended_diagnostics: self.extended_diagnostics,
            key_ref_info: KeyRefInfo::default(),
            _phantom1: &PhantomData,
            _phantom2: &PhantomData,
        };
        context.validate_any_type(self.types.root())
    }
}

struct ValidationContext<'s, 'c, 'r, C: TypeContainer<'c>, R: LqReader<'r>> {
    types: &'s C,
    config: Config,
    reader: &'s mut R,
    extended_diagnostics: bool,
    key_ref_info: KeyRefInfo,
    _phantom1: &'c PhantomData<()>,
    _phantom2: &'r PhantomData<()>,
}

impl<'s, 'c, 'r, C: TypeContainer<'c>, R: LqReader<'r>> Context<'r>
    for ValidationContext<'s, 'c, 'r, C, R>
{
    type Reader = R;

    fn validate(&mut self, reference: &TypeRef) -> Result<(), LqError> {
        let any_type = self.types.require_type(reference)?;
        self.validate_any_type(any_type)
    }

    fn validate_any_type(&mut self, any_type: &AnyType) -> Result<(), LqError> {
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
    }

    fn compare(
        &self,
        reference: &TypeRef,
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

    fn key_ref_info(&mut self) -> &mut KeyRefInfo {
        &mut self.key_ref_info
    }
}

fn enrich_validation_error<'a, R: LqReader<'a>>(
    err: LqError,
    mut reader: R,
    r#type: &AnyType,
) -> LqError {
    let mut reader_content_description = reader.clone();
    let content_description = reader_content_description.read_content_description();

    let mut reader_next_10_bytes = reader.clone();
    let next_20_bytes = reader_next_10_bytes.read_slice(10);

    let value = Value::de_serialize(&mut reader);
    let value_str = match value {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{:?}", err),
    };

    let new_msg = format!(
        "{}. Extended diagnostics:\n\n - Type to validate: {:?}\n\n - \
         Content description: {:?}\n\n - 10 bytes: {:?}\n\n - Value: {}",
        err.msg(),
        r#type,
        content_description,
        next_20_bytes,
        value_str
    );
    err.with_msg(new_msg)
}
