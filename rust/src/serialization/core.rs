use std::borrow::Cow;
use std::error::Error;
use std::fmt::Display;
use std::io::Write;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TypeId(u8);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TypeBlock(u8);

pub trait Writer {
    fn write<T: TypeWriter>(&mut self, item: &T::Item) -> Result<(), LqError>;
}

pub trait Reader<'a> {
    fn read<T: TypeReader<'a>>(&mut self) -> Result<T::Item, LqError>;
}

pub trait TypeReader<'a> {
    type Item;

    fn read<T: BinaryReader<'a>>(id: TypeId, reader: &mut T) -> Result<Self::Item, LqError>;
}

pub trait TypeWriter {
    type Item: ?Sized;

    fn write<'b, Writer: BinaryWriter<'b> + 'b>(
        writer: Writer,
        item: &Self::Item,
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

pub trait BinaryWriter<'a> {
    type Writer: Write;
    fn begin(self, id: TypeId) -> Result<&'a mut Self::Writer, LqError>;
}

pub trait BinaryReader<'a>: std::io::Read {
    fn read_u8(&mut self) -> Result<u8, LqError>;
    fn read_slice(&mut self, len: usize) -> Result<&'a [u8], LqError>;
}

#[derive(Debug)]
pub struct LqError {
    pub msg: Cow<'static, str>,
}

impl Error for LqError {}

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

impl TypeBlock {
    pub const fn new(block: u8) -> Self {
        Self(block)
    }

    #[inline]
    pub fn id(&self) -> u8 {
        self.0
    }
}

impl TypeId {
    pub const fn new(id: u8) -> TypeId {
        TypeId(id)
    }

    pub const fn from_block(block: TypeBlock, remainder: u8) -> TypeId {
        TypeId(block.0 * 16u8 + remainder)
    }

    pub fn id(&self) -> u8 {
        self.0
    }

    pub fn block(&self) -> TypeBlock {
        TypeBlock::new((self.0 & 0xF0) / 16)
    }

    pub fn remainder(&self) -> u8 {
        self.0 & 0x0F
    }
}
