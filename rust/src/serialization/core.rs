use std::borrow::Cow;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TypeId(u8);

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TypeBlock(u8);

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TypeRemainder(u8);

pub trait Writer {
    fn write<T: Serializer>(&mut self, item: &T::Item) -> Result<(), LqError>;
}

pub trait Reader<'a> {
    fn read<T: DeSerializer<'a>>(&mut self) -> Result<T::Item, LqError>;

    fn skip(&mut self, number_of_fields: usize) -> Result<(), LqError>;
}

pub trait DeSerializer<'a> {
    type Item;

    fn de_serialize<T: BinaryReader<'a>>(reader: &mut T) -> Result<Self::Item, LqError>;

    /// Skips the data of this type. For types that do not contain embedded data (like
    ///  bool or int) it's the same as `read` (this is what this default implementation does).
    fn skip<T: BinaryReader<'a>>(reader: &mut T) -> Result<SkipMore, LqError> {
        Self::de_serialize(reader)?;
        Result::Ok(SkipMore::new(0))
    }   
}

/// Sometimes a data is not alone but contains embedded items. For example the
/// optional type contains the present value (if present) - or structs contain
/// 0-n fields.
///
/// When skipping we also might want too skip those embedded data.
pub struct SkipMore(usize);

impl SkipMore {
    pub fn new(number_of_additional_items: usize) -> Self {
        Self(number_of_additional_items)
    }

    pub fn number_of_additional_items(&self) -> usize {
        self.0
    }
}

pub trait Serializer {
    type Item: ?Sized;

    fn serialize<T: BinaryWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError>;
}

pub trait BinaryWriter: std::io::Write {
    fn write_u8(&mut self, data: u8) -> Result<(), LqError>;
    fn write_slice(&mut self, buf: &[u8]) -> Result<(), LqError>;
    fn type_id(&mut self, id: TypeId) -> Result<(), LqError>;
}

pub trait BinaryReader<'a>: std::io::Read {
    fn read_u8(&mut self) -> Result<u8, LqError>;
    fn read_slice(&mut self, len: usize) -> Result<&'a [u8], LqError>;
    fn type_id(&mut self) -> Result<TypeId, LqError>;
    fn preview_type_id(&self) -> Result<TypeId, LqError>;
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

    pub fn remainder(&self) -> TypeRemainder {
        TypeRemainder(self.0 & 0x0F)
    }
}

impl TypeRemainder {
    pub fn id(&self) -> u8 {
        self.0
    }
}
