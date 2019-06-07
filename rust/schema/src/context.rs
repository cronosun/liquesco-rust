use crate::any_type::AnyType;
use crate::core::Config;
use crate::core::TypeRef;
use liquesco_common::error::LqError;
use liquesco_serialization::core::LqReader;
use std::cmp::Ordering;

// TODO: Rename to `ValidationContext`
/// Data used for type validation.
pub trait Context<'a>: CmpContext<'a> {
    fn validate(&mut self, reference: &TypeRef) -> Result<(), LqError>;

    fn validate_any_type(&mut self, any_type: &AnyType) -> Result<(), LqError>;

    fn reader(&mut self) -> &mut Self::Reader;

    fn config(&self) -> &Config;

    /// Returns the key ref info currently active (top op stack).
    fn key_ref_info(&self, level: u32) -> Option<KeyRefInfo>;

    /// Pushes a new key ref info on top of the stack.
    fn push_key_ref_info(&mut self, info: KeyRefInfo);

    /// Pops the key ref info from top of the stack. Returns an error when you try to pop
    /// when the stack is already empty.
    fn pop_key_ref_info(&mut self) -> Result<KeyRefInfo, LqError>;
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
#[derive(Clone, Copy, PartialOrd, PartialEq, Hash)]
pub struct KeyRefInfo {
    map_len: u32,
}

impl KeyRefInfo {
    pub fn new(map_len: u32) -> KeyRefInfo {
        Self { map_len }
    }

    /// The length of the map that's currently being validated.
    pub fn map_len(&self) -> u32 {
        self.map_len
    }
}
