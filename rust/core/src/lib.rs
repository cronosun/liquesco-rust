#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate from_variants;

pub mod common;
pub mod schema;
pub mod serde;
pub mod serialization;

#[cfg(test)]
pub mod tests;
