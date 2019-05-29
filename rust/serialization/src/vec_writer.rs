use crate::core::LqWriter;
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

impl std::io::Write for VecWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.extend_from_slice(buf);
        Result::Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Result::Ok(())
    }
}

impl LqWriter for VecWriter {
    fn write_u8(&mut self, data: u8) -> Result<(), LqError> {
        self.data.push(data);
        Result::Ok(())
    }

    fn write_slice(&mut self, buf: &[u8]) -> Result<(), LqError> {
        self.data.extend_from_slice(buf);
        Result::Ok(())
    }
}

impl VecWriter {
    /// Finishes the writer and returns the written data as `Vec<u8>`.
    pub fn finish(self) -> Vec<u8> {
        self.data
    }
}
