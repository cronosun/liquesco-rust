use crate::core::Config;
use crate::core::TypeRef;
use liquesco_common::error::LqError;
use liquesco_serialization::core::LqReader;
use std::cmp::Ordering;

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

    // TODO: Deprecated
    fn anchor_index(&self) -> Option<u32>;
    // TODO: Deprecated
    fn set_anchor_index(&mut self, value: Option<u32>);

    // TODO: Deprecated
    fn max_used_anchor_index(&self) -> Option<u32>;
    // TODO: Deprecated
    fn set_max_used_anchor_index(&mut self, value: Option<u32>);

    fn key_ref_info(&mut self) -> &mut KeyRefInfo;
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
