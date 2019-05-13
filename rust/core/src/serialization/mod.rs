pub mod core;
pub mod slice_reader;
pub mod vec_writer;
pub mod value;
pub mod value_into;

pub(self) mod major_types;
pub(self) mod common_binary;

pub mod boolean;
pub mod unicode;
pub mod binary;
pub mod option;
pub mod seq;
pub mod enumeration;
pub mod uint;
pub mod sint;
pub mod uuid;
pub mod float;