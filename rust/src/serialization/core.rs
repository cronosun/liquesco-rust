use std::fmt::Display;
use std::error::Error;
use std::borrow::Cow;
use std::io::Write;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TypeId(pub u8);

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

pub trait Serializable {
    fn serialize<T: Writer>(&self, writer: &mut T) -> Result<(), LqError>;
}

pub trait DeSerializable<'a> {
    fn de_serialize<T: Reader<'a>>(reader: &'a mut T) -> Result<Self, LqError>
    where
        Self: Sized;
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

