#[macro_use]
extern crate derive_new;

pub mod any_type;
pub mod core;
pub mod doc_type;
pub mod identifier;
pub mod schema;
pub mod schema_builder;
#[cfg(test)]
pub mod tests;

pub mod anchors;
pub mod ascii;
pub mod boolean;
pub mod enumeration;
pub mod float;
pub mod option;
pub mod reference;
pub mod seq;
pub mod sint;
pub mod structure;
pub mod uint;
pub mod unicode;
pub mod uuid;