use crate::common::error::LqError;
use crate::serialization::core::BinaryReader;

/// Wraps a `BinaryReader` so we can use it for dynamic calls (e.g. in traits).
pub struct DynReader<'a> {
    reader: &'a mut BinaryReader<'a>,
}

impl<'a, 'b> DynReader<'a> {
    pub fn from<T : BinaryReader<'a>>(reader : &'a mut T) -> Self {
        Self { reader  }
    }
}

impl<'a, 'b> From<&'a mut BinaryReader<'a>> for DynReader<'a> {
    fn from(reader: &'a mut BinaryReader<'a>) -> Self {
        Self { reader }
    }
}

impl<'a> BinaryReader<'a> for DynReader<'a> {
    fn peek_u8(&self) -> Result<u8, LqError> {
        self.reader.peek_u8()
    }

    fn read_u8(&mut self) -> Result<u8, LqError> {
        self.reader.read_u8()
    }

    fn read_slice(&mut self, len: usize) -> Result<&'a [u8], LqError> {
        self.reader.read_slice(len)
    }
}

impl<'a> std::io::Read for DynReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}
