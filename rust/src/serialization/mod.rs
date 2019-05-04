pub mod core;
pub mod slice_reader;
pub mod vec_writer;
pub mod value;
pub mod value_into;
pub mod dyn_reader;

pub(self) mod type_ids;
pub(self) mod binary;
pub(self) mod util;

pub mod tbool;
pub mod tutf8;
pub mod tbinary;
pub mod toption;
pub mod tlist;
pub mod tenum;
pub mod tuint;
pub mod tsint;
pub mod tuuid;