pub mod serde;
pub mod value;

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
