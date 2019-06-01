#[macro_use]
extern crate derive_new;

pub mod any_type;
pub mod core;
pub mod context;
pub mod identifier;
pub mod metadata;
pub mod schema;
pub mod schema_anchors;
pub mod schema_builder;

pub mod anchors;
pub mod ascii;
pub mod binary;
pub mod boolean;
pub mod enumeration;
pub mod float;
pub mod option;
pub mod range;
pub mod reference;
pub mod seq;
pub mod map;
pub mod root_map;
pub mod sint;
pub mod structure;
pub mod uint;
pub mod unicode;
pub mod uuid;
pub mod key_ref;
