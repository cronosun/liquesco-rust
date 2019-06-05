use std::io::Write;
use crate::core::LqWriter;
use liquesco_common::error::LqError;

pub struct WriteWriter<'a, W> {
    write : &'a mut W,
}

impl<'a, W : Write> WriteWriter<'a, W> {
    pub fn new(write : &'a mut W) -> Self {
        Self {
            write
        }
    }
}

impl<'a, W : Write> std::io::Write for WriteWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.write.flush()
    }
}

impl<'a, W : Write> LqWriter for WriteWriter<'a, W> {
    fn write_u8(&mut self, data: u8) -> Result<(), LqError> {
        self.write.write_all(&[data])?;
        Ok(())
    }

    fn write_slice(&mut self, buf: &[u8]) -> Result<(), LqError> {
        self.write.write_all(buf)?;
        Ok(())
    }
}
