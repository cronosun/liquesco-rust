use crate::serialization::util::io_result;
use crate::serialization::util::try_from_int_result;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use integer_encoding::VarIntReader;
use integer_encoding::VarIntWriter;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;

use enum_repr::EnumRepr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TypeId(u8);

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TypeHeader(u8);

/// Allowed range: 0 to 9 (inclusive)
#[EnumRepr(type = "u8")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum LengthMarker {
    Len0 = 0,
    Len1 = 1,
    Len2 = 2,
    Len4 = 3,
    Len8 = 4,
    VarInt = 5,
    /// container type: Followed by var int for number of items and var int for self length
    ContainerVarIntVarInt = 6,
    // container type: Followed by var int for number of items. Has no self length.
    ConainerVarIntEmpty = 7,
    // container type: Has one item and no self length.
    ConainerOneEmpty = 8,
    // reserved for further extensions
    Reserved = 9,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ContainerHeader {
    number_of_items: usize,
    self_length: usize,
}

pub trait Writer {
    fn write<T: Serializer>(&mut self, item: &T::Item) -> Result<(), LqError>;
}

pub trait Reader<'a> {
    fn read<T: DeSerializer<'a>>(&mut self) -> Result<T::Item, LqError>;
}

pub trait DeSerializer<'a> {
    type Item;

    fn de_serialize<T: BinaryReader<'a>>(reader: &mut T) -> Result<Self::Item, LqError>;   
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

pub trait BinaryWriter: std::io::Write + Sized {
    fn write_u8(&mut self, data: u8) -> Result<(), LqError>;
    fn write_slice(&mut self, buf: &[u8]) -> Result<(), LqError>;

    fn write_varint(&mut self, value: usize) -> Result<(), LqError> {
        io_result(VarIntWriter::write_varint(self, value)).map(|_| {})
    }

    fn write_u16(&mut self, data: u16) -> Result<(), LqError> {
        io_result(WriteBytesExt::write_u16::<LittleEndian>(self, data))
    }

    fn write_u32(&mut self, data: u32) -> Result<(), LqError> {
        io_result(WriteBytesExt::write_u32::<LittleEndian>(self, data))
    }

    fn write_u64(&mut self, data: u64) -> Result<(), LqError> {
        io_result(WriteBytesExt::write_u64::<LittleEndian>(self, data))
    }

    fn write_i8(&mut self, data: i8) -> Result<(), LqError> {
        io_result(WriteBytesExt::write_i8(self, data))
    }

    fn write_i16(&mut self, data: i16) -> Result<(), LqError> {
        io_result(WriteBytesExt::write_i16::<LittleEndian>(self, data))
    }

    fn write_i32(&mut self, data: i32) -> Result<(), LqError> {
        io_result(WriteBytesExt::write_i32::<LittleEndian>(self, data))
    }

    fn write_i64(&mut self, data: i64) -> Result<(), LqError> {
        io_result(WriteBytesExt::write_i64::<LittleEndian>(self, data))
    }

    fn write_header(&mut self, header: TypeHeader) -> Result<(), LqError> {
        BinaryWriter::write_u8(self, header.id())
    }

    fn write_header_u8(&mut self, type_id: TypeId, len: u8) -> Result<(), LqError> {
        self.write_header_usize(type_id, len as usize)
    }

    fn write_header_u16(&mut self, type_id: TypeId, len: u16) -> Result<(), LqError> {
        self.write_header_usize(type_id, len as usize)
    }

    fn write_header_u32(&mut self, type_id: TypeId, len: u32) -> Result<(), LqError> {
        self.write_header_usize(type_id, try_from_int_result(usize::try_from(len))?)
    }

    fn write_header_u64(&mut self, type_id: TypeId, len: u64) -> Result<(), LqError> {
        self.write_header_usize(type_id, try_from_int_result(usize::try_from(len))?)
    }

    fn write_header_usize(&mut self, type_id: TypeId, len: usize) -> Result<(), LqError> {
        let marker = match len {
            0 => LengthMarker::Len0,
            1 => LengthMarker::Len1,
            2 => LengthMarker::Len2,
            4 => LengthMarker::Len4,
            8 => LengthMarker::Len8,
            _ => LengthMarker::VarInt,
        };
        self.write_header(TypeHeader::new(marker, type_id))?;
        if  marker==LengthMarker::VarInt {
            BinaryWriter::write_u8(self, len as u8)?;
        }
        Result::Ok(())
    }

    fn write_container_header(
        &mut self,
        type_id: TypeId,
        conainer_header: ContainerHeader,
    ) -> Result<(), LqError> {
        if conainer_header.self_length == 0 && conainer_header.number_of_items == 1 {
            self.write_header(TypeHeader::new(LengthMarker::ConainerOneEmpty, type_id))
        } else if conainer_header.self_length == 0 {
            self.write_header(TypeHeader::new(LengthMarker::ConainerVarIntEmpty, type_id))?;
            self.write_varint(conainer_header.number_of_items)
        } else {
            self.write_header(TypeHeader::new(
                LengthMarker::ContainerVarIntVarInt,
                type_id,
            ))?;
            self.write_varint(conainer_header.number_of_items)?;
            self.write_varint(conainer_header.self_length)
        }
    }
}

pub trait BinaryReader<'a>: std::io::Read + Sized {
    fn read_u8(&mut self) -> Result<u8, LqError>;
    fn read_slice(&mut self, len: usize) -> Result<&'a [u8], LqError>;

    fn read_varint(&mut self) -> Result<usize, LqError> {
        io_result(VarIntReader::read_varint(self))
    }

    fn read_u16(&mut self) -> Result<u16, LqError> {
        io_result(ReadBytesExt::read_u16::<LittleEndian>(self))
    }

    fn read_u32(&mut self) -> Result<u32, LqError> {
        io_result(ReadBytesExt::read_u32::<LittleEndian>(self))
    }

    fn read_u64(&mut self) -> Result<u64, LqError> {
        io_result(ReadBytesExt::read_u64::<LittleEndian>(self))
    }

    fn read_header(&mut self) -> Result<TypeHeader, LqError> {
        let header_byte = BinaryReader::read_u8(self)?;
        Result::Ok(TypeHeader::from_u8(header_byte))
    }

    fn read_i8(&mut self) -> Result<i8, LqError> {
        io_result(ReadBytesExt::read_i8(self))
    }

    fn read_i16(&mut self) -> Result<i16, LqError> {
        io_result(ReadBytesExt::read_i16::<LittleEndian>(self))
    }

    fn read_i32(&mut self) -> Result<i32, LqError> {
        io_result(ReadBytesExt::read_i32::<LittleEndian>(self))
    }

    fn read_i64(&mut self) -> Result<i64, LqError> {
        io_result(ReadBytesExt::read_i64::<LittleEndian>(self))
    }

    fn read_header_const(&mut self, len: u8) -> Result<TypeId, LqError> {
        let header = self.read_header_u64()?;
        let real_len = header.1;
        if u64::from(len) != real_len {
            LqError::err_new(format!(
                "Invalid type length, expecting {:?} but have {:?}",
                len, real_len
            ))
        } else {
            Result::Ok(header.0)
        }
    }

    fn read_header_u64(&mut self) -> Result<(TypeId, u64), LqError> {
        let (id, size) = self.read_header_usize()?;
        Result::Ok((id, try_from_int_result(u64::try_from(size))?))
    }

    fn read_header_usize(&mut self) -> Result<(TypeId, usize), LqError> {
        let header = self.read_header()?;
        let marker = header.length_marker();
        let length = match marker {
            LengthMarker::Len0 => Result::Ok(0),
            LengthMarker::Len1 => Result::Ok(1),
            LengthMarker::Len2 => Result::Ok(2),
            LengthMarker::Len4 => Result::Ok(4),
            LengthMarker::Len8 => Result::Ok(8),
            LengthMarker::VarInt => Result::Ok(self.read_varint()?),
            LengthMarker::ContainerVarIntVarInt | LengthMarker::ConainerVarIntEmpty | LengthMarker::ConainerOneEmpty => LqError::err_static(
                "This is a container; the called function cannot be used for containers.",
            ),
            LengthMarker::Reserved => LqError::err_static(
                "Encoding error. Got the reserved value (reserved for future use).",
            ),
        }?;
        Result::Ok((header.type_id(), length))
    }

    fn read_header_container(&mut self, header : TypeHeader) -> Result<ContainerHeader, LqError> {
        match header.length_marker() {
            LengthMarker::ContainerVarIntVarInt  => {
                let number_of_items = self.read_varint()?;
                let self_length = self.read_varint()?;
                Result::Ok(
                    ContainerHeader {
                        number_of_items,
                        self_length,
                    }
                )
            },
            LengthMarker::ConainerOneEmpty => {
                 Result::Ok(
                    ContainerHeader {
                        number_of_items : 1,
                        self_length : 0,
                    }
                )
            },
            LengthMarker::ConainerVarIntEmpty => {
                let number_of_items = self.read_varint()?;
                  Result::Ok(
                    ContainerHeader {
                        number_of_items,
                        self_length : 0,
                    }
                )
            },
            _ => {
                LqError::err_static("Not a container type")
            }
        }
    }
    
    /// Skips a type and all embedded items.
    fn skip(&mut self) ->  Result<(), LqError> {
        let header = self.read_header()?;
        match header.length_marker() {
            LengthMarker::ContainerVarIntVarInt | LengthMarker::ConainerVarIntEmpty | LengthMarker::ConainerOneEmpty => {
                // it's a container type
                let container_info = self.read_header_container(header)?;
                self.skip_bytes(container_info.self_length)?;
                let number_of_embedded_types = container_info.number_of_items();
                if number_of_embedded_types>0 {
                    for _ in 0..number_of_embedded_types {
                        self.skip()?;
                    }
                }
                Result::Ok(())
            },
             LengthMarker::Len0 => self.skip_bytes(0),
            LengthMarker::Len1 => self.skip_bytes(1),
            LengthMarker::Len2 => self.skip_bytes(2),
            LengthMarker::Len4 => self.skip_bytes(4),
            LengthMarker::Len8 => self.skip_bytes(8),
            LengthMarker::VarInt => {
                let number_to_skip = self.read_varint()?;
                self.skip_bytes(number_to_skip)
            },
            LengthMarker::Reserved => {
                LqError::err_static("Reserved entry! Reserved for further extensions")
            }
        }        
    }

    fn skip_bytes(&mut self,  number_of_bytes : usize) -> Result<(), LqError> {
        for _ in 0..number_of_bytes {
            self.read_u8()?;
        }
        Result::Ok(())
    }
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

impl TypeId {
    pub const fn new(id: u8) -> TypeId {
        TypeId(id)
    }

    pub fn id(self) -> u8 {
        self.0
    }
}

impl TypeHeader {
    pub fn new(len: LengthMarker, id: TypeId) -> TypeHeader {
        let len_byte = len as u8;
        TypeHeader(len_byte * 10 + id.0)
    }

    pub(crate) fn from_u8(byte: u8) -> TypeHeader {
        TypeHeader(byte)
    }

    pub fn length_marker(self) -> LengthMarker {
        let len_byte = self.0 / 10;
        LengthMarker::from_repr(len_byte).unwrap()
    }

    pub fn type_id(self) -> TypeId {
        let id_byte = self.0 % 10;
        TypeId(id_byte)
    }

    pub fn id(self) -> u8 {
        self.0
    }
}

impl ContainerHeader {
    pub fn new(number_of_items: usize, self_length: usize) -> Self {
        Self {
            number_of_items,
            self_length,
        }
    }

    pub fn self_length(&self) -> usize {
        self.self_length
    }

    pub fn number_of_items(&self) -> usize {
        self.number_of_items
    }
}
