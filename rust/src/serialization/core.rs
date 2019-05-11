use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use enum_repr::EnumRepr;
use varuint::*;

use crate::common::error::LqError;
use crate::common::internal_utils::io_result;

/// The major type can be within 0-24 (inclusive).
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct MajorType(u8);

/// Combines the `MajorType` and the `ContentInfo`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TypeHeader(u8);

/// Information about the content: How many bytes does the type take and are there
/// embedded values?
///
/// Allowed range: 0 to 12 (inclusive; 13 items)
#[EnumRepr(type = "u8")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum ContentInfo {
    /// An item with no length and no embedded items.
    Len0 = 0,
    /// An item with length 1 and no embedded items.
    Len1 = 1,
    /// An item with length 2 and no embedded items.
    Len2 = 2,
    /// An item with length 3 and no embedded items.
    Len4 = 3,
    /// An item with length 4 and no embedded items.
    Len8 = 4,
    /// An item with length 16 and no embedded items.
    Len16 = 5,
    /// An item with length 32 and no embedded items.    
    VarInt = 6,
    /// Container: #embedded items: 1; self length: 0.
    ContainerOneEmpty = 7,
    /// Container: #embedded items: 2; self length: 0.
    ContainerTwoEmpty = 8,
    /// Container: #embedded items: 1; self length: 1.
    ContainerOneOne = 9,
    /// Container: #embedded items: encoded as var int; self length: 0.
    ContainerVarIntEmpty = 10,
    /// Container: #embedded items: encoded as var int; self length: encoded as var int.    
    ContainerVarIntVarInt = 11,
    /// reserved for further extensions
    Reserved = 12,
}

/// Description about the content of one type: How many bytes and how many embedded items?
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ContentDescription {
    number_of_embedded_values: u32,
    self_length: u64,
}

pub trait DeSerializer<'a> {
    type Item;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError>;
}

pub trait Serializer {
    type Item: ?Sized;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError>;
}

pub trait LqWriter: std::io::Write + Sized {
    fn write_u8(&mut self, data: u8) -> Result<(), LqError>;
    fn write_slice(&mut self, buf: &[u8]) -> Result<(), LqError>;

    fn write_varint_u64(&mut self, value: u64) -> Result<(), LqError> {
        io_result(WriteVarint::<u64>::write_varint(self, value).map(|_| {}))
    }

    fn write_varint_u32(&mut self, value: u32) -> Result<(), LqError> {
        io_result(WriteVarint::<u32>::write_varint(self, value).map(|_| {}))
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

    fn write_f32(&mut self, data: f32) -> Result<(), LqError> {
        io_result(WriteBytesExt::write_f32::<LittleEndian>(self, data))
    }

    fn write_f64(&mut self, data: f64) -> Result<(), LqError> {
        io_result(WriteBytesExt::write_f64::<LittleEndian>(self, data))
    }

    fn write_header(&mut self, header: TypeHeader) -> Result<(), LqError> {
        LqWriter::write_u8(self, header.id())
    }

    fn write_content_description(
        &mut self,
        major_type: MajorType,
        content_description: &ContentDescription,
    ) -> Result<(), LqError> {
        let self_len = content_description.self_length;
        let number_of_embedded_values = content_description.number_of_embedded_values;
        if number_of_embedded_values == 0 {
            let marker = match self_len {
                0 => ContentInfo::Len0,
                1 => ContentInfo::Len1,
                2 => ContentInfo::Len2,
                4 => ContentInfo::Len4,
                8 => ContentInfo::Len8,
                16 => ContentInfo::Len16,
                _ => ContentInfo::VarInt,
            };
            self.write_header(TypeHeader::new(marker, major_type))?;
            if marker == ContentInfo::VarInt {
                LqWriter::write_varint_u64(self, self_len)?;
            }
            Result::Ok(())
        } else if self_len == 0 && number_of_embedded_values == 1 {
            self.write_header(TypeHeader::new(ContentInfo::ContainerOneEmpty, major_type))
        } else if self_len == 0 && number_of_embedded_values == 2 {
            self.write_header(TypeHeader::new(ContentInfo::ContainerTwoEmpty, major_type))
        } else if self_len == 1 && number_of_embedded_values == 1 {
            self.write_header(TypeHeader::new(ContentInfo::ContainerOneOne, major_type))
        } else if self_len == 0 {
            self.write_header(TypeHeader::new(
                ContentInfo::ContainerVarIntEmpty,
                major_type,
            ))?;
            self.write_varint_u32(number_of_embedded_values)
        } else {
            self.write_header(TypeHeader::new(
                ContentInfo::ContainerVarIntVarInt,
                major_type,
            ))?;
            self.write_varint_u32(number_of_embedded_values)?;
            self.write_varint_u64(self_len)
        }
    }
}

pub trait LqReader<'a>: std::io::Read {
    fn peek_u8(&self) -> Result<u8, LqError>;
    fn read_u8(&mut self) -> Result<u8, LqError>;
    fn read_slice(&mut self, len: usize) -> Result<&'a [u8], LqError>;

    /// creates a clone that shares the underlying buffer but
    /// has an independent read offset (cursor).
    fn clone(&self) -> Self
    where
        Self: Sized;

    fn peek_header(&self) -> Result<TypeHeader, LqError> {
        let value = self.peek_u8()?;
        Result::Ok(TypeHeader::from_u8(value))
    }

    fn read_varint_u32(&mut self) -> Result<u32, LqError> {
        io_result(ReadVarint::<u32>::read_varint(self))
    }

    fn read_varint_u64(&mut self) -> Result<u64, LqError> {
        io_result(ReadVarint::<u64>::read_varint(self))
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

    fn read_type_header(&mut self) -> Result<TypeHeader, LqError> {
        let header_byte = LqReader::read_u8(self)?;
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

    fn read_f32(&mut self) -> Result<f32, LqError> {
        io_result(ReadBytesExt::read_f32::<LittleEndian>(self))
    }

    fn read_f64(&mut self) -> Result<f64, LqError> {
        io_result(ReadBytesExt::read_f64::<LittleEndian>(self))
    }

    fn read_expect_content_description(
        &mut self,
        self_len: u64,
        number_of_embedded_values: u32,
    ) -> Result<MajorType, LqError> {
        let type_header = self.read_type_header()?;
        let content_description = self.read_content_description_given_type_header(type_header)?;

        if content_description.number_of_embedded_values != number_of_embedded_values {
            return LqError::err_new(format!(
                "Expecting to have {:?} embedded values for this type; have {:?} embedded values.",
                number_of_embedded_values, content_description.number_of_embedded_values
            ));
        }

        if content_description.self_length != self_len {
            return LqError::err_new(format!(
                "Expecting to a length of {:?} bytes but have a length of {:?} bytes.",
                self_len, content_description.self_length
            ));
        }

        Result::Ok(type_header.major_type())
    }

    fn read_content_description(&mut self) -> Result<ContentDescription, LqError> {
        let type_header = self.read_type_header()?;
        self.read_content_description_given_type_header(type_header)
    }

    fn read_content_description_given_type_header(
        &mut self,
        header: TypeHeader,
    ) -> Result<ContentDescription, LqError> {
        match header.content_info() {
            ContentInfo::Len0 => Result::Ok(ContentDescription {
                number_of_embedded_values: 0,
                self_length: 0,
            }),
            ContentInfo::Len1 => Result::Ok(ContentDescription {
                number_of_embedded_values: 0,
                self_length: 1,
            }),
            ContentInfo::Len2 => Result::Ok(ContentDescription {
                number_of_embedded_values: 0,
                self_length: 2,
            }),
            ContentInfo::Len4 => Result::Ok(ContentDescription {
                number_of_embedded_values: 0,
                self_length: 4,
            }),
            ContentInfo::Len8 => Result::Ok(ContentDescription {
                number_of_embedded_values: 0,
                self_length: 8,
            }),
            ContentInfo::Len16 => Result::Ok(ContentDescription {
                number_of_embedded_values: 0,
                self_length: 16,
            }),
            ContentInfo::VarInt => {
                let self_length = self.read_varint_u64()?;
                Result::Ok(ContentDescription {
                    number_of_embedded_values: 0,
                    self_length,
                })
            }
            ContentInfo::ContainerVarIntVarInt => {
                let number_of_embedded_values = self.read_varint_u32()?;
                let self_length = self.read_varint_u64()?;
                Result::Ok(ContentDescription {
                    number_of_embedded_values,
                    self_length,
                })
            }
            ContentInfo::ContainerOneEmpty => Result::Ok(ContentDescription {
                number_of_embedded_values: 1,
                self_length: 0,
            }),
            ContentInfo::ContainerOneOne => Result::Ok(ContentDescription {
                number_of_embedded_values: 1,
                self_length: 1,
            }),
            ContentInfo::ContainerTwoEmpty => Result::Ok(ContentDescription {
                number_of_embedded_values: 2,
                self_length: 0,
            }),
            ContentInfo::ContainerVarIntEmpty => {
                let number_of_embedded_values = self.read_varint_u32()?;
                Result::Ok(ContentDescription {
                    number_of_embedded_values,
                    self_length: 0,
                })
            }
            ContentInfo::Reserved => LqError::err_static(
                "Cannot decode content description: Got the reserved content info 
                (must not be found; this is reserved for future extensions).",
            ),
        }
    }

    /// Skips a type and all embedded items.
    fn skip(&mut self) -> Result<(), LqError> {
        let header = self.read_type_header()?;
        let content_description = self.read_content_description_given_type_header(header)?;
        // first skip "myself"
        let self_length = content_description.self_length;
        if self_length > 0 {
            self.skip_bytes_u64(content_description.self_length)?;
        }
        // then skip all embedded values
        let number_of_embedded_values = content_description.number_of_embedded_values;
        if number_of_embedded_values > 0 {
            for _ in 0..number_of_embedded_values {
                self.skip()?;
            }
        }
        Result::Ok(())
    }

    /// Same as `skip` but can skip multiple values.
    fn skip_n_values(&mut self, number_of_values: usize) -> Result<(), LqError> {
        for _ in 0..number_of_values {
            self.skip()?;
        }
        Result::Ok(())
    }

    /// Same as `skip` but can skip multiple values.
    fn skip_n_values_u32(&mut self, number_of_values: u32) -> Result<(), LqError> {
        for _ in 0..number_of_values {
            self.skip()?;
        }
        Result::Ok(())
    }

    fn skip_bytes(&mut self, number_of_bytes: usize) -> Result<(), LqError> {
        for _ in 0..number_of_bytes {
            self.read_u8()?;
        }
        Result::Ok(())
    }

    fn skip_bytes_u64(&mut self, number_of_bytes: u64) -> Result<(), LqError> {
        for _ in 0..number_of_bytes {
            self.read_u8()?;
        }
        Result::Ok(())
    }
}

impl MajorType {
    pub const fn new(id: u8) -> MajorType {
        MajorType(id)
    }

    pub fn id(self) -> u8 {
        self.0
    }
}

const FACTOR: u8 = 13;

impl TypeHeader {
    pub fn new(len: ContentInfo, id: MajorType) -> TypeHeader {
        let len_byte = len as u8;
        TypeHeader(id.0 * FACTOR + len_byte)
    }

    pub(crate) fn from_u8(byte: u8) -> TypeHeader {
        TypeHeader(byte)
    }

    pub fn content_info(self) -> ContentInfo {
        let len_byte = self.0 % FACTOR;
        ContentInfo::from_repr(len_byte).unwrap()
    }

    pub fn major_type(self) -> MajorType {
        let id_byte = self.0 / FACTOR;
        MajorType(id_byte)
    }

    pub fn id(self) -> u8 {
        self.0
    }
}

impl ContentDescription {
    /// How many bytes does this type occupy itself (excluding) embedded types.
    pub fn self_length(&self) -> u64 {
        self.self_length
    }

    /// How many embedded values does this value have.
    pub fn number_of_embedded_values(&self) -> u32 {
        self.number_of_embedded_values
    }

    pub fn set_number_of_embedded_values(&mut self, number_of_embedded_values: u32) {
        self.number_of_embedded_values = number_of_embedded_values;
    }

    pub fn set_self_length(&mut self, self_length: u64) {
        self.self_length = self_length;
    }

    pub fn new_self_length(self_length: u64) -> Self {
        ContentDescription {
            self_length,
            number_of_embedded_values: 0,
        }
    }

    pub fn new_number_of_embedded_values(number_of_embedded_values: u32) -> Self {
        ContentDescription {
            self_length: 0,
            number_of_embedded_values,
        }
    }
}

impl Default for ContentDescription {
    fn default() -> Self {
        ContentDescription {
            self_length: 0,
            number_of_embedded_values: 0,
        }
    }
}
