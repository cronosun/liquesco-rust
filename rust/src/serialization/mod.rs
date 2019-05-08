pub mod core;
pub mod slice_reader;
pub mod vec_writer;
pub mod value;
pub mod value_into;
pub mod dyn_reader;

pub(self) mod major_types;
pub(self) mod binary;

pub mod tbool;
pub mod tunicode;
pub mod tbinary;
pub mod toption;
pub mod tseq;
pub mod tenum;
pub mod tuint;
pub mod tsint;
pub mod tuuid;
pub mod tfloat;