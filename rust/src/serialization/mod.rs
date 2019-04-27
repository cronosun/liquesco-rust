pub mod core;
pub mod slice_reader;
pub mod vec_writer;
//pub mod types;

pub(crate) mod type_ids;
pub(crate) mod binary;
pub(crate) mod util;

pub mod tbool;
pub mod tutf8;
pub mod tbinary;
pub mod toption;
pub mod tstruct;
pub mod tenum;

#[cfg(test)]
pub mod test;