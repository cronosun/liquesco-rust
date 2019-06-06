use crate::any_type::AnyType;
use crate::core::Config;
use crate::core::TypeRef;
use liquesco_common::error::LqError;
use liquesco_serialization::core::LqReader;
use std::cmp::Ordering;

// TODO: Rename to `ValidationContext`
/// Data used for type validation.
pub trait Context<'a> : CmpContext<'a> {
    fn validate(&mut self, reference: &TypeRef) -> Result<(), LqError>;

    fn validate_any_type(&mut self, any_type: &AnyType) -> Result<(), LqError>;    

    fn reader(&mut self) -> &mut Self::Reader;

    fn config(&self) -> &Config;

    fn key_ref_info(&mut self) -> &mut KeyRefInfo;
}

/// The context for type compare. It's a simplified version of `Context`.
pub trait CmpContext<'a> {       
    type Reader: LqReader<'a>;

    /// See `Type::compare`.
    fn compare(
        &self,
        reference: &TypeRef,
        r1: &mut Self::Reader,
        r2: &mut Self::Reader,
    ) -> Result<Ordering, LqError>;
}

/// Information used for key ref validation.
#[derive(Clone)]
pub struct KeyRefInfo {
    map_len: Option<u32>,
}

impl Default for KeyRefInfo {
    fn default() -> Self {
        Self { map_len: None }
    }
}

impl KeyRefInfo {
    pub fn set_map_len(&mut self, map_len: Option<u32>) {
        self.map_len = map_len;
    }

    /// The length of the map that's currently being validated. Returns `None` if there's no map
    /// being processed.
    pub fn map_len(&self) -> Option<u32> {
        self.map_len
    }

    pub fn restore_from(&mut self, from: KeyRefInfo) {
        self.map_len = from.map_len
    }
}
