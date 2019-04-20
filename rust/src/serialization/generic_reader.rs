// TODO: Remove
use crate::serialization::core::TypeId;
use crate::serialization::core::Type;
use crate::serialization::core::LqError;
use std::marker::PhantomData;
use std::io::Read;
use crate::serialization::core::BinaryReader;
use crate::serialization::core::Reader;
use std::io::Write;

pub trait AbstractReader<'a> : Reader<'a> + BinaryReader + Read {
    fn data(&self) -> &[u8];
    fn offset(&self) -> usize;
    fn increment_offset(&mut self, by : usize);
}

pub struct GenericReader<'a, T : AbstractReader<'a>> {
    reader : T,
    phantom_a : PhantomData<&'a T>
}

impl<'a, T : AbstractReader<'a>> Reader<'a> for GenericReader<'a, T> {
    fn read<TType: Type<'a>>(&'a mut self) -> Result<TType::ReadItem, LqError> {
        let original_offset = self.reader.offset();
        let result = self.read_no_error::<TType>();
        if result.is_err() {
            // TODO: Add additional output
           result
        } else {
            result
        }
    }
}

impl<'a, T : AbstractReader<'a>> GenericReader<'a, T> {
    fn read_no_error<TType: Type<'a>>(&'a mut self) -> Result<TType::ReadItem, LqError> {
        let type_id_byte = self.read_u8()?;
        let type_id = TypeId(type_id_byte);

        TType::read(type_id, self)
    }

    /// Makes sure the reader has been read completely and there's no additional data.
    pub fn finish(&self) -> Result<(), LqError> {
        if self.reader.offset() != self.reader.data().len() {
            LqError::err_static(
                "There's addtional data not read from any. The any data must have been comsumed 
            entirely (for security reasons).",
            )
        } else {
            Result::Ok(())
        }
    }
}

impl<'a, T : AbstractReader<'a>> Read for GenericReader<'a, T> {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        let len = buf.len();
        let slice = self.read_slice(len).map_err(|err| {
            std::io::Error::new(std::io::ErrorKind::Other, err)
        })?;
        buf.write(slice)
    }
}

impl<'a, T : AbstractReader<'a>> BinaryReader for GenericReader<'a, T> {
    #[inline]
    fn read_u8(&mut self) -> Result<u8, LqError> {
        let len = self.reader.data().len();
        if self.reader.offset() >= len {
            LqError::err_static("End of reader")
        } else {
            let value = self.reader.data()[self.reader.offset()];
            self.reader.increment_offset(1);
            Result::Ok(value)
        }
    }

    #[inline]
    fn read_slice(&mut self, len: usize) -> Result<&[u8], LqError> {
        let data_len = self.reader.data().len();
        if self.reader.offset() + len > data_len {
            LqError::err_static("End of reader")
        } else {
            let end_index = self.reader.offset() + len;
            let value = &self.reader.data()[self.reader.offset()..end_index];
            self.reader.increment_offset(len);
            Result::Ok(value)
        }
    }
}
