use crate::serialization::core::LqReader;
use crate::common::error::LqError;
use std::io::Read;
use std::io::Write;

pub struct SliceReader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> From<&'a [u8]> for SliceReader<'a> {
    fn from(data: &'a [u8]) -> Self {
        SliceReader { data, offset: 0 }
    }
}

impl<'a> From<&'a Vec<u8>> for SliceReader<'a> {
    fn from(data : &'a Vec<u8>) -> Self {
        SliceReader { data : data.as_slice(), offset : 0 }
    }
}

impl<'a> SliceReader<'a> {    
    /// Makes sure the reader has been read completely and there's no additional data.
    pub fn finish(&self) -> Result<(), LqError> {
        if self.offset != self.data.len() {
            LqError::err_static(
                "There's addtional data not read from any. The any data must have been comsumed 
            entirely (for security reasons).",
            )
        } else {
            Result::Ok(())
        }
    }
}

impl<'a> LqReader<'a> for SliceReader<'a> {
    #[inline]
    fn read_u8(&mut self) -> Result<u8, LqError> {
        let len = self.data.len();
        if self.offset >= len {
            LqError::err_static("End of reader")
        } else {
            let value = self.data[self.offset];
            self.offset += 1;
            Result::Ok(value)
        }
    }

    #[inline]
    fn peek_u8(&self) -> Result<u8, LqError> {
        let len = self.data.len();
        if self.offset >= len {
            LqError::err_static("End of reader")
        } else {
            let value = self.data[self.offset];
            Result::Ok(value)
        }
    }

    #[inline]
    fn read_slice(&mut self, len: usize) -> Result<&'a [u8], LqError> {
        let data_len = self.data.len();
        if self.offset + len > data_len {
            LqError::err_static("End of reader")
        } else {
            let end_index = self.offset + len;
            let data = self.data;
            let value = &data[self.offset..end_index];
            self.offset += len;
            Result::Ok(value)
        }
    }
}

impl<'a> Read for SliceReader<'a> {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        let len = buf.len();
        let slice = self
            .read_slice(len)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        buf.write(slice)
    }
}
