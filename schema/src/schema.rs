use crate::any_type::AnyType;
use crate::context::CmpContext;
use crate::context::KeyRefInfo;
use crate::context::ValidationContext;
use crate::core::Schema;
use crate::core::TypeContainer;
use crate::core::TypeRef;
use crate::core::{Config, Type};
use crate::identifier::Identifier;
use crate::metadata::MetadataSetter;
use crate::schema_builder::{BuildsOwnSchema, SchemaBuilder};
use crate::types::key_ref::TKeyRef;
use crate::types::root_map::TRootMap;
use liquesco_common::error::LqError;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::core::LqReader;
use liquesco_serialization::value::Value;
use smallvec::SmallVec;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::marker::PhantomData;

/// Builds the liquesco schema schema.
pub fn schema_schema<B>(mut builder: B) -> Result<B::TTypeContainer, LqError>
where
    B: SchemaBuilder<'static>,
{
    let root_ref = builder.add_unwrap(
        "schema_root",
        TKeyRef::default().with_doc(
            "This references the root \
             type. The root type is the type schema validation begins with.",
        ),
    );
    let any_type = AnyType::build_schema(&mut builder);
    let identifier = Identifier::build_schema(&mut builder);
    let root = builder.add_unwrap(
        "schema",
        TRootMap::new(root_ref, identifier, any_type).with_doc("The liquesco schema."),
    );

    builder.finish(root)
}

pub struct DefaultSchema<'a, C: TypeContainer + Clone> {
    types: Cow<'a, C>,
    extended_diagnostics: bool,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, C: TypeContainer + Clone> DefaultSchema<'a, C> {
    pub fn with_extended_diagnostics(mut self, extended_diagnostics: bool) -> Self {
        self.extended_diagnostics = extended_diagnostics;
        self
    }
}

impl<'a, C: TypeContainer + Clone> Schema for DefaultSchema<'a, C> {
    fn validate<'r, R: LqReader<'r>>(&self, config: Config, reader: &mut R) -> Result<(), LqError> {
        self.validate_internal(config, reader)
    }

    fn compare<'r, R: LqReader<'r>>(
        &self,
        type_ref: &TypeRef,
        r1: &mut R,
        r2: &mut R,
    ) -> Result<Ordering, LqError> {
        let type_container: &C = &self.types;
        let cmp_context = DefaultCmpContext {
            types: type_container,
            extended_diagnostics: self.extended_diagnostics,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        };
        cmp_context.compare(type_ref, r1, r2)
    }
}

impl<'a, C: TypeContainer + Clone> TypeContainer for DefaultSchema<'a, C> {
    fn maybe_type(&self, reference: &TypeRef) -> Option<&AnyType> {
        self.types.maybe_type(reference)
    }

    fn root(&self) -> &TypeRef {
        self.types.root()
    }

    fn identifier(&self, reference: &TypeRef) -> Result<Cow<Identifier>, LqError> {
        self.types.identifier(reference)
    }

    fn require_type(&self, reference: &TypeRef) -> Result<&AnyType, LqError> {
        self.types.require_type(reference)
    }
}

impl<'a, T: TypeContainer + Clone> From<T> for DefaultSchema<'a, T> {
    fn from(container: T) -> Self {
        Self::new(Cow::Owned(container))
    }
}

impl<'a, T: TypeContainer + Clone> From<&'a T> for DefaultSchema<'a, T> {
    fn from(container: &'a T) -> Self {
        Self::new(Cow::Borrowed(container))
    }
}

impl<'a, C: TypeContainer + Clone> DefaultSchema<'a, C> {
    pub fn new<IntoC: Into<Cow<'a, C>>>(types: IntoC) -> Self {
        Self {
            types: types.into(),
            extended_diagnostics: false,
            _phantom: PhantomData,
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
        let type_container: &C = &self.types;
        let mut context = DefaultValidationContext {
            types: type_container,
            config,
            reader,
            extended_diagnostics: self.extended_diagnostics,
            key_ref_info: SmallVec::new(),
            _phantom1: &PhantomData,
            _phantom2: &PhantomData,
        };
        context.validate(self.types.root())
    }
}

struct DefaultValidationContext<'s, 'c, 'r, C: TypeContainer, R: LqReader<'r>> {
    types: &'s C,
    config: Config,
    reader: &'s mut R,
    extended_diagnostics: bool,
    /// The key ref info. Note: We use a smallvec of 4, since it's very rare that there are
    /// ever more than 4 levels.
    key_ref_info: SmallVec<[KeyRefInfo; 4]>,
    _phantom1: &'c PhantomData<()>,
    _phantom2: &'r PhantomData<()>,
}

impl<'s, 'c, 'r, C: TypeContainer, R: LqReader<'r>> CmpContext<'r>
    for DefaultValidationContext<'s, 'c, 'r, C, R>
{
    type Reader = R;

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
}

impl<'s, 'c, 'r, C: TypeContainer, R: LqReader<'r>> ValidationContext<'r>
    for DefaultValidationContext<'s, 'c, 'r, C, R>
{
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

    fn reader(&mut self) -> &mut Self::Reader {
        &mut self.reader
    }

    fn config(&self) -> &Config {
        &self.config
    }

    fn key_ref_info(&self, level: u32) -> Option<KeyRefInfo> {
        let len = self.key_ref_info.len();
        let usize_level = usize::try_from(level).ok();
        if let Some(usize_level) = usize_level {
            if usize_level < len {
                Some(self.key_ref_info[len - usize_level - 1])
            } else {
                None
            }
        } else {
            None
        }
    }

    fn push_key_ref_info(&mut self, info: KeyRefInfo) {
        self.key_ref_info.push(info);
    }

    fn pop_key_ref_info(&mut self) -> Result<KeyRefInfo, LqError> {
        let len = self.key_ref_info.len();
        if len == 0 {
            LqError::err_new(
                "You're trying to pop from ref info stack but the ref info \
                 stack is empty. This is a bug in the liquesco implementation.",
            )
        } else {
            Ok(self.key_ref_info.remove(len - 1))
        }
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

struct DefaultCmpContext<'a, 'r, C: TypeContainer, R: LqReader<'r>> {
    types: &'a C,
    extended_diagnostics: bool,
    _phantom1: PhantomData<R>,
    _phantom2: PhantomData<&'r ()>,
}

impl<'a, 'r, C: TypeContainer, R: LqReader<'r>> CmpContext<'r> for DefaultCmpContext<'a, 'r, C, R> {
    type Reader = R;

    fn compare(
        &self,
        reference: &TypeRef,
        r1: &mut Self::Reader,
        r2: &mut Self::Reader,
    ) -> Result<Ordering, LqError> {
        if let Some(any_type) = self.types.maybe_type(reference) {
            if self.extended_diagnostics {
                let saved_reader1 = r1.clone();
                let saved_reader2 = r2.clone();
                let result = any_type.compare(self, r1, r2);
                match result {
                    Err(err) => Err(enrich_cmp_error(err, saved_reader1, saved_reader2)),
                    Ok(ok) => Ok(ok),
                }
            } else {
                any_type.compare(self, r1, r2)
            }
        } else {
            LqError::err_new(format!(
                "Type (reference {:?}) not found. \
                 Unable to validate (compare).",
                reference
            ))
        }
    }
}

fn enrich_cmp_error<'a, R: LqReader<'a>>(err: LqError, mut r1: R, mut r2: R) -> LqError {
    let value1 = Value::de_serialize(&mut r1);
    let value1_str = match value1 {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{:?}", err),
    };

    let value2 = Value::de_serialize(&mut r2);
    let value2_str = match value2 {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{:?}", err),
    };

    let new_msg = format!(
        "{}. Extended diagnostics:\n\n - Compare LHS: {:?}\n\n - \
         Compare RHS: {:?}",
        err.msg(),
        value1_str,
        value2_str
    );
    err.with_msg(new_msg)
}
