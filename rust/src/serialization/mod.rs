pub mod core;
pub mod slice_reader;
pub mod vec_writer;
pub mod value;
pub mod value_into;
pub mod dyn_reader;

pub(crate) mod type_ids;
pub(crate) mod binary;
pub(crate) mod util;

pub mod tbool;
pub mod tutf8;
pub mod tbinary;
pub mod toption;
pub mod tlist;
pub mod tenum;
pub mod tuint;
pub mod tsint;
pub mod tuuid;