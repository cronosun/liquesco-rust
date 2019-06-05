use crate::core::LqWriter;
use crate::core::ToVecLqWriter;
use liquesco_common::error::LqError;

/// Implements the `LqWriter` that writes into a `Vec<u8>`.
pub struct VecWriter {
    data: Vec<u8>,
}

impl Default for VecWriter {
    fn default() -> Self {
        VecWriter { data: Vec::new() }
    }
}

impl<'a> std::io::Write for VecWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.extend_from_slice(buf);
        Result::Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Result::Ok(())
    }
}

impl<'a> LqWriter for VecWriter {
    fn write_u8(&mut self, data: u8) -> Result<(), LqError> {
        self.data.push(data);
        Result::Ok(())
    }

    fn write_slice(&mut self, buf: &[u8]) -> Result<(), LqError> {
        self.data.extend_from_slice(buf);
        Result::Ok(())
    }
}

impl<'a> ToVecLqWriter for VecWriter {
    fn into_vec(self) -> Vec<u8> {
        self.data
    }
}
