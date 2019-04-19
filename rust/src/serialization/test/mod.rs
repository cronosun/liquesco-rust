use crate::serialization::core::VecWriter;

pub mod binary;
pub mod utf8;

fn new_writer() -> VecWriter {
    VecWriter::default()
}