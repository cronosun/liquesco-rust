#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate from_variants;

pub mod common;
pub mod serialization;
pub mod schema;
pub mod serde;

#[cfg(test)]
pub mod tests;