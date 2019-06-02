use crate::metadata::WithMetadata;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};

use crate::any_type::AnyType;
use crate::context::Context;
use crate::identifier::Identifier;
use crate::identifier::StrIdentifier;
use liquesco_common::error::LqError;
use liquesco_serialization::core::LqReader;

use serde::{Deserialize, Serialize};
use std::rc::Rc;
use serde::export::Formatter;
use serde::export::fmt::Error;
use std::borrow::Cow;

/// A single type in the schema; for example an integer or a structure.
pub trait Type: Debug + WithMetadata {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>;

    /// Compares r1 to r2. It's expected that you call this function only
    /// on data that has been validated successfully (if you call this on
    /// invalid data the behaviour is undefined).
    ///
    /// Important: The state of the given reader `r1` and `r2` is undefined
    /// unless this function returns `Ordering::Equal`. When inequality has
    /// been detected not all data will be read. Only reads all data when
    /// `Ordering::Equal` is returned.
    ///
    /// - Greater: if r1 > r2
    /// - Less: if r1 < r2
    /// - Equal: if r1 == r2
    fn compare<'c, C>(
        &self,
        context: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>;

    /// Returns the embedded references by index (starting at index 0).
    /// Returns `None` if there are no more references (does not contain
    /// gaps).
    ///
    /// This is mostly used internally; usually you get embedded references
    /// by the appropriate methods.
    fn reference(&self, _: usize) -> Option<&TypeRef>;

    /// Sets the reference. This has to succeed when `reference` returns a non-empty type ref.
    /// It has to fail when reference returns an empty type ref.
    fn set_reference(&mut self, index : usize, type_ref : TypeRef) -> Result<(), LqError>;
}

/// Configuration used for validation.
#[derive(new)]
pub struct Config {
    /// When this is false, structures and enum variants cannot be extended. It's a
    /// validation error when a structure has more fields than defined in the schema; it's
    /// a validation error when an enum variant has more values than defined in the schema.
    ///
    /// This should be true if you want to accept data that has been constructed for a
    /// later schema version.
    #[new(value = "false")]
    pub no_extension: bool,

    /// When this is true, wrong anchor ordering is ignored. Also unused anchors are allowed. You
    /// usually want this to be false.
    #[new(value = "false")]
    pub weak_reference_validation: bool,
}

impl Config {
    /// This returns true if e.g. extensions in structures (e.g. have more fields than defined in
    /// the schema) is not allowed.
    pub fn no_extension(&self) -> bool {
        self.no_extension
    }

    pub fn strict() -> Self {
        Self {
            no_extension: true,
            weak_reference_validation: false,
        }
    }
}

/// References a single type within a schema.
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum TypeRef {
    /// This is the reference used for serialization (a serialized schema only uses numbers).
    Numerical(u32),
    /// This is the reference used when building a schema.
    Identifier(StrIdentifier<'static>)
}

impl TypeRef {
    /// Constructs a new type reference. This should usually never be used by user code,
    /// it's only to be used by `TypeContainer` implementations.
    pub fn new_numerical(id: u32) -> Self {
        TypeRef::Numerical(id)
    }
}

/// Contains multiple `Type` that can be got using a `TypeRef`.
pub trait TypeContainer<'a> {
    /// Returns a `Type` if contained within this container.
    fn maybe_type(&self, reference: &TypeRef) -> Option<&AnyType<'a>>;

    /// Returns the root type.
    fn root(&self) -> &AnyType<'a>;

    /// Returns a `Type` if contained within this container.
    fn require_type(&self, reference: &TypeRef) -> Result<&AnyType<'a>, LqError> {
        if let Some(present) = self.maybe_type(reference) {
            Ok(present)
        } else {
            LqError::err_new(format!("There's no such type referenced by {}", reference))
        }
    }
}

/// A schema. Can be used to validate data.
pub trait Schema<'a>: TypeContainer<'a> {
    fn validate<'r, R: LqReader<'r>>(&self, config: Config, reader: &mut R) -> Result<(), LqError>;
}


/// Need custom serde.
impl serde::Serialize for TypeRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        match self {
            TypeRef::Identifier(id) => {
                // TODO
                /*S::Error::custom(format!("Unable to serialize identifier type ref ({}). Type ref \
                need to be converted to numerical representation before it can \
                be serialized.", id))*/
                let msg = format!("Unable to serialize identifier type ref ({:?}). Type ref \
                need to be converted to numerical representation before it can \
                be serialized.", id);
                panic!(msg)
            },
            TypeRef::Numerical(num) => {
                serializer.serialize_u32(*num)
            }

        }
    }
}

/// Need custom serde.
impl<'de> serde::Deserialize<'de> for TypeRef {
    fn deserialize<D>(deserializer: D) -> Result<TypeRef, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u32(TypeRefVisitor)
    }
}

struct TypeRefVisitor;

impl<'de> serde::de::Visitor<'de> for TypeRefVisitor {
    type Value = TypeRef;

    fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
        formatter.write_str("Expecting a u32 for type ref.")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where
        E: serde::de::Error, {
        Ok(TypeRef::new_numerical(v))
    }
}

impl Display for TypeRef {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            TypeRef::Identifier(id) => {
                write!(f, "{}", id)
            },
            TypeRef::Numerical(num) => {
                write!(f, "#{}", num)
            }
        }
    }
}
