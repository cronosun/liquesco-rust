use smallvec::SmallVec;
use std::borrow::Cow;
use crate::serialization::uuid::Uuid;
use std::cmp::Ordering;
use std::fmt::Debug;

use crate::common::error::LqError;
use crate::schema::any_type::AnyType;
use crate::serialization::core::LqReader;

pub type Doc<'a> = Option<Cow<'a, str>>;
pub type Implements = Option<SmallVec<[Uuid; 2]>>;

pub trait Type: Debug /*+ TypeDoc*/ {
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
}

const EMPTY : &[Uuid] = &[];

pub trait TypeDoc {
    /// Type documentation. Optional.
    fn doc(&self) -> Option<&str> {
        None
    }

    /// A set of items this type implements. What is this used for? It can 
    /// be used to identify compatible types company- or world-wide.
    fn implements(&self) -> &[Uuid] {
        EMPTY
    }
}

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

#[derive(new)]
pub struct Config {
    #[new(value = "false")]
    pub no_extension: bool,
}

impl Config {
    /// This returns true if e.g. extensions in structures (e.g. have more fields than defined in
    /// the schema) is not allowed.
    pub fn no_extension(&self) -> bool {
        self.no_extension
    }

    pub fn strict() -> Self {
        Self { no_extension: true }
    }
}

/// References a single type within a schema.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TypeRef(pub usize);

/// Contains multiple `Type` that can be got using a `TypeRef`.
pub trait TypeContainer<'a> {
    /// Returns a `Type` if contained within this container.
    fn maybe_type(&self, reference: TypeRef) -> Option<&AnyType<'a>>;
}

pub trait Schema<'a>: TypeContainer<'a> {
    fn validate<'r, R: LqReader<'r>>(&self, config: Config, reader: &mut R) -> Result<(), LqError>;
    fn main_type(&self) -> TypeRef;
}
