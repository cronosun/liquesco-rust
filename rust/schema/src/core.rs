use crate::metadata::WithMetadata;
use std::cmp::Ordering;
use std::fmt::Debug;

use crate::any_type::AnyType;
use liquesco_common::error::LqError;
use liquesco_serialization::core::LqReader;

use serde::{Deserialize, Serialize};

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
    fn reference(&self, _: usize) -> Option<TypeRef>;
}

/// Data used for type validation.
pub trait Context<'a> {
    type Reader: LqReader<'a>;

    fn validate(&mut self, reference: TypeRef) -> Result<(), LqError>;

    /// See `Type::compare`.
    fn compare(
        &self,
        reference: TypeRef,
        r1: &mut Self::Reader,
        r2: &mut Self::Reader,
    ) -> Result<Ordering, LqError>;

    fn reader(&mut self) -> &mut Self::Reader;

    fn config(&self) -> &Config;

    fn anchor_index(&self) -> Option<u32>;
    fn set_anchor_index(&mut self, value: Option<u32>);

    fn max_used_anchor_index(&self) -> Option<u32>;
    fn set_max_used_anchor_index(&mut self, value: Option<u32>);
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
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct TypeRef(pub u32);

/// Contains multiple `Type` that can be got using a `TypeRef`.
pub trait TypeContainer<'a> {
    /// Returns a `Type` if contained within this container.
    fn maybe_type(&self, reference: TypeRef) -> Option<&AnyType<'a>>;

    // TODO: Why not returning th master type ID here?
}

/// A schema. Can be used to validate data.
pub trait Schema<'a>: TypeContainer<'a> {
    fn validate<'r, R: LqReader<'r>>(&self, config: Config, reader: &mut R) -> Result<(), LqError>;
    fn main_type(&self) -> TypeRef;
}
