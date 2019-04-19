use std::fmt::Display;
use std::error::Error;
use std::borrow::Cow;
use std::io::Read;
use std::io::Write;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TypeId(pub u8);

pub struct SliceReader<'a> {
    data: &'a [u8],
    pub offset: usize,
}

pub trait Writer {
    fn write<'a, T: Type<'a>>(&mut self, item: &T::WriteItem) -> Result<(), LqError>;
}

pub trait Reader<'a> {
    fn read<T: Type<'a>>(&'a mut self) -> Result<T::ReadItem, LqError>;
}

pub struct VecWriter {
    data: Vec<u8>,
}

impl Default for VecWriter {
    fn default() -> Self {
        VecWriter { data: Vec::new() }
    }
}

pub trait Type<'a> {
    type ReadItem;
    type WriteItem: ?Sized;

    fn read<Reader : BinaryReader>(
        id: TypeId, reader: &'a mut Reader) -> Result<Self::ReadItem, LqError>;
    fn write<'b, Writer: BinaryWriter<'b> + 'b>(
        writer: Writer,
        item: &Self::WriteItem,
    ) -> Result<(), LqError>;
}

impl<'a> From<&'a [u8]> for SliceReader<'a> {
    fn from(data: &'a [u8]) -> Self {
        SliceReader { data, offset: 0 }
    }
}

pub trait Serializable {
    fn serialize<T: Writer>(&self, writer: &mut T) -> Result<(), LqError>;
}

pub trait DeSerializable<'a> {
    fn de_serialize<T: Reader<'a>>(reader: &'a mut T) -> Result<Self, LqError>
    where
        Self: Sized;
}

impl<'a> Reader<'a> for SliceReader<'a> {
    fn read<T: Type<'a>>(&'a mut self) -> Result<T::ReadItem, LqError> {
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

impl<'a> SliceReader<'a> {
    fn read_no_error<T: Type<'a>>(&'a mut self) -> Result<T::ReadItem, LqError> {
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

impl Writer for VecWriter {
    fn write<'a, T: Type<'a>>(&mut self, item: &T::WriteItem) -> Result<(), LqError> {
        let header_writer = HeaderWriterStruct {
            data: &mut self.data,
        };
        T::write(header_writer, item)
    }
}

impl VecWriter {
    pub fn finish(self) -> Vec<u8> {
        self.data
    }
}

struct HeaderWriterStruct<'a> {
    data: &'a mut Vec<u8>,
}

impl<'a> BinaryWriter<'a> for HeaderWriterStruct<'a> {
    type Writer = Vec<u8>;

    fn begin(self, id: TypeId) -> Result<&'a mut Self::Writer, LqError> {
        self.data.push(id.0);
        Result::Ok(self.data)
    }
}

pub trait BinaryWriter<'a> {
    type Writer: Write;
    fn begin(self, id: TypeId) -> Result<&'a mut Self::Writer, LqError>;
}

pub trait BinaryReader: std::io::Read {
    fn read_u8(&mut self) -> Result<u8, LqError>;
    fn read_slice(& mut self, len: usize) -> Result<& [u8], LqError>;
}

// TODO: Remove
pub struct ReadResult<Data> {
    pub num_read: usize,
    pub data: Data,
}

impl<Data> ReadResult<Data> {
    pub fn new(num_read: usize, data: Data) -> Self {
        ReadResult { num_read, data }
    }

    pub fn new_ok(num_read: usize, data: Data) -> Result<Self, LqError> {
        Result::Ok(Self::new(num_read, data))
    }
}

#[derive(Debug)]
pub struct LqError {
    pub msg: Cow<'static, str>,
}

impl Error for LqError {
}

impl Display for LqError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LqError({:?})", self.msg)
    }
}

impl LqError {
    pub fn err_static<Ok>(string: &'static str) -> Result<Ok, LqError> {
        Result::Err(LqError { msg: string.into() })
    }

    pub fn new<T: Into<Cow<'static, str>>>(msg: T) -> Self {
        LqError { msg: msg.into() }
    }

    pub fn err_new<Ok, T: Into<Cow<'static, str>>>(msg: T) -> Result<Ok, Self> {
        Result::Err(Self::new(msg))
    }

    pub fn with_msg<T: Into<Cow<'static, str>>>(mut self, msg: T) -> LqError {
        self.msg = msg.into();
        self
    }
}

impl<'a> BinaryReader for SliceReader<'a> {
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
    fn read_slice(&mut self, len: usize) -> Result<&[u8], LqError> {
        let data_len = self.data.len();
        if self.offset + len > data_len {
            LqError::err_static("End of reader")
        } else {
            let end_index = self.offset + len;
            let value = &self.data[self.offset..end_index];
            self.offset = self.offset + len;
            Result::Ok(value)
        }
    }
}

impl<'a> Read for SliceReader<'a> {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        let len = buf.len();
        let slice = self.read_slice(len).map_err(|err| {
            std::io::Error::new(std::io::ErrorKind::Other, err)
        })?;
        buf.write(slice)
    }
}
