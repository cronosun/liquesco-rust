use crate::identifier::Identifier;
use liquesco_serialization::uuid::Uuid;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::Debug;

use crate::any_type::AnyType;
use liquesco_common::error::LqError;
use liquesco_serialization::core::LqReader;

use serde::{Deserialize, Serialize};

pub trait Type: Debug {
    fn doc(&self) -> &Doc {
        // types usually have no documentation. We use a special wrapper that adds
        // documentation to a type. See `DocType`.
        EMPTY_DOC
    }

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

    // Returns the embedded references by index (starting at index 0).
    // Returns `None` if there are no more references (does not contain
    // gaps).
    //
    // This is mostly used internally; usually you get embedded references
    // by the appropriate methods. 
    fn reference(&self, _ : usize) -> Option<TypeRef>;
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
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct TypeRef(pub u32);

/// Contains multiple `Type` that can be got using a `TypeRef`.
pub trait TypeContainer<'a> {
    /// Returns a `Type` if contained within this container.
    fn maybe_type(&self, reference: TypeRef) -> Option<&AnyType<'a>>;
}

pub trait Schema<'a>: TypeContainer<'a> {
    fn validate<'r, R: LqReader<'r>>(&self, config: Config, reader: &mut R) -> Result<(), LqError>;
    fn main_type(&self) -> TypeRef;
}

pub const DOC_MIN_LEN_UTF8_BYTES: usize = 1;
pub const DOC_MAX_LEN_UTF8_BYTES: usize = 4000;

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Doc<'a> {
    name: Option<Identifier<'a>>,
    description: Option<Cow<'a, str>>,
    implements: Option<Implements>,
}

const EMPTY_DOC: &Doc = &Doc::empty();

impl<'a> Doc<'a> {
    pub const fn empty() -> Self {
        Self {
            name: None,
            description: None,
            implements: None,
        }
    }

    pub fn name(&self) -> Option<&Identifier<'a>> {
        if let Some(identifier) = &self.name {
            Some(&identifier)
        } else {
            None
        }
    }

    pub fn description(&self) -> Option<&str> {
        if let Some(desc) = &self.description {
            Some(desc)
        } else {
            None
        }
    }

    pub fn implements(&self) -> &[Uuid] {
        if let Some(implements) = &self.implements {
            &implements.0
        } else {
            &[]
        }
    }

    pub fn set_name<I>(&mut self, name: I)
    where
        I: Into<Identifier<'a>>,
    {
        self.name = Some(name.into());
    }

    pub fn set_description<D>(&mut self, description: D)
    where
        D: Into<Cow<'a, str>>,
    {
        self.description = Some(description.into());
    }

    pub fn add_implements<U>(&mut self, uuid: U) -> Result<(), LqError>
    where
        U: Into<Uuid>,
    {
        if let None = self.implements {
            self.implements = Option::Some(Implements::try_new(&[uuid.into()])?);
        } else {
            let mut implements = self.implements.take().unwrap();
            implements.add(uuid.into())?;
            self.implements = Some(implements);
        }
        Ok(())
    }
}

impl<'a> Default for Doc<'a> {
    fn default() -> Self {
        Doc::empty()
    }
}

pub const MIN_IMPLEMENTS_ELEMENTS: usize = 1;
pub const MAX_IMPLEMENTS_ELEMENTS: usize = 255;

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Implements(Vec<Uuid>);

impl Implements {
    pub fn try_new(implements: &[Uuid]) -> Result<Self, LqError> {
        let number = implements.len();
        if number < MIN_IMPLEMENTS_ELEMENTS {
            LqError::err_new("You need at least one element in 'implements'.")
        } else if number > MAX_IMPLEMENTS_ELEMENTS {
            LqError::err_new(format!(
                "There are too many implements elements. Maximum is {:?}; got {:?} elements.",
                MAX_IMPLEMENTS_ELEMENTS, number
            ))
        } else {
            Ok(Implements(Vec::from(implements)))
        }
    }

    pub fn add(&mut self, implements: Uuid) -> Result<(), LqError> {
        let number = self.0.len() + 1;
        if number > MAX_IMPLEMENTS_ELEMENTS {
            LqError::err_new(format!(
                "There are too many implements elements. Maximum is {:?}; got {:?} elements.",
                MAX_IMPLEMENTS_ELEMENTS, number
            ))
        } else {
            self.0.push(implements);
            Ok(())
        }
    }
}
