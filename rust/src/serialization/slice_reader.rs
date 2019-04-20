use crate::serialization::core::TypeReader;
use std::io::Read;
use std::io::Write;
use crate::serialization::core::BinaryReader;
use crate::serialization::core::TypeId;
use crate::serialization::core::LqError;
use crate::serialization::core::Reader;

pub struct MemReader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> From<&'a [u8]> for MemReader<'a> {
    fn from(data: &'a [u8]) -> Self {
        MemReader { data , offset: 0 }
    }
}

impl<'a> Reader<'a> for MemReader<'a> {
    fn read<T: TypeReader<'a>>(&mut self) -> Result<T::Item, LqError> {
        let original_offset = self.offset;
        let result = self.read_no_error::<T>();
        if result.is_err() {
            // TODO
           result
        } else {
            result
        }
/*
        self.read_no_error::<T>().map_err(|original| {
            // add some message details
            let original_msg = &original.msg;
            let data_len = self.data.len();
            let offset_to_use = if self.offset < data_len {
                self.offset
            } else {
                data_len
            };
            let data = &self.data[offset_to_use..];
            let new_message = format!(
                "Error reading any data at offset {:?}: \"{:?}\". Binary at offset {:?} is {:?}.",
                self.offset, original_msg, offset_to_use, data
            );
            original.with_msg(new_message)
        })*/
    }
}

impl<'a> MemReader<'a> {
    fn read_no_error<T: TypeReader<'a>>(&mut self) -> Result<T::Item, LqError> {
        let type_id_byte = self.read_u8()?;
        let type_id = TypeId(type_id_byte);

        T::read(type_id, self)
    }

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


impl<'a> BinaryReader<'a> for MemReader<'a> {
    #[inline]
    fn read_u8(&mut self) -> Result<u8, LqError> {
        let len = self.data.len();
        if self.offset >= len {
            LqError::err_static("End of reader")
        } else {
            let value = self.data[self.offset];
            self.offset = self.offset + 1;
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
            self.offset = self.offset + len;
            Result::Ok(value)
        }
    }
}

impl<'a> Read for MemReader<'a> {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        let len = buf.len();
        let slice = self.read_slice(len).map_err(|err| {
            std::io::Error::new(std::io::ErrorKind::Other, err)
        })?;
        buf.write(slice)
    }
}