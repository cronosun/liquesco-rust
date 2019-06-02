#[macro_use]
extern crate derive_new;

pub mod any_type;
pub mod context;
pub mod core;
pub mod identifier;
pub mod metadata;
// TODO: pub mod schema;
// TODO: pub mod schema_anchors;
pub mod schema_builder;

pub mod ascii;
pub mod binary;
pub mod boolean;
pub mod enumeration;
pub mod float;
pub mod key_ref;
pub mod map;
pub mod option;
pub mod range;
pub mod root_map;
pub mod seq;
pub mod sint;
pub mod structure;
pub mod uint;
pub mod unicode;
pub mod uuid;
