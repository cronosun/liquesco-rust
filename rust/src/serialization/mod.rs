pub mod core;
pub mod slice_reader;
//pub mod vec_reader;
//pub mod abstract_reader;
pub(crate) mod types;
pub(crate) mod binary;
pub(crate) mod util;

//pub mod custom;

pub mod tbool;
pub mod tutf8;
pub mod tbinary;
pub mod toption;
pub mod tstruct;

#[cfg(test)]
pub mod test;