use std::borrow::Cow;
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
    fn read<T: Type<'a>>(&mut self) -> Result<T::ReadItem, LqError>;
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

    fn read(id: TypeId, data: &'a [u8]) -> Result<ReadResult<Self::ReadItem>, LqError>;
    fn write<'b, Writer: HeaderWriter<'b> + 'b>(
        writer: Writer,
        item: &Self::WriteItem,
    ) -> Result<(), LqError>;
}

impl<'a> From<&'a [u8]> for SliceReader<'a> {
    fn from(data: &'a [u8]) -> Self {
        SliceReader {
            data,
            offset: 0,
        }
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
    fn read<T: Type<'a>>(&mut self) -> Result<T::ReadItem, LqError> {
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
        })
    }
}

impl<'a> SliceReader<'a> {
    fn read_no_error<T: Type<'a>>(&mut self) -> Result<T::ReadItem, LqError> {
        let data_len = self.data.len();
        let data_with_offset = if self.offset + 1 > data_len {
            return LqError::err_static("Out of bounds (end of data)");
        } else if self.offset == data_len {
            &[0; 0]
        } else {
            &self.data[self.offset + 1..]
        };
        let type_id = TypeId(self.data[self.offset]);

        let result = T::read(type_id, data_with_offset)?;
        self.offset = self.offset + result.num_read + 1;
        Result::Ok(result.data)
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

impl<'a> HeaderWriter<'a> for HeaderWriterStruct<'a> {
    type Writer = Vec<u8>;

    fn type_id(self, id: TypeId) -> &'a mut Self::Writer {
        self.data.push(id.0);
        self.data
    }
}

pub trait HeaderWriter<'a> {
    type Writer: Write;
    fn type_id(self, id: TypeId) -> &'a mut Self::Writer;
}

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
