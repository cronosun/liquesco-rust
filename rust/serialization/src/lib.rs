#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate from_variants;

pub mod serde;
pub mod serialization;
pub mod value;

#[cfg(test)]
pub mod tests;

pub mod core;
pub mod slice_reader;
pub mod vec_writer;

pub(crate) mod common_binary;
pub(crate) mod major_types;

pub mod binary;
pub mod boolean;
pub mod enumeration;
pub mod float;
pub mod option;
pub mod seq;
pub mod sint;
pub mod uint;
pub mod unicode;
pub mod uuid;