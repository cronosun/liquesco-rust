use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::LengthMarker;
use crate::common::error::LqError;
use crate::serialization::core::Serializer;
use crate::serialization::type_ids::TYPE_UINT;

pub struct TUInt;
pub struct TUIntU8;

impl<'a> DeSerializer<'a> for TUInt {
    type Item = u64;

    fn de_serialize<R: BinaryReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let header = reader.read_header()?;
        if header.type_id() != TYPE_UINT {
            return LqError::err_static("Given type is not an unsigned integer type");
        }
        match header.length_marker() {
            LengthMarker::Len0 => Result::Ok(0),
            LengthMarker::Len1 => Result::Ok(u64::from(reader.read_u8()?)),
            LengthMarker::Len2 => Result::Ok(u64::from(reader.read_u16()?)),
            LengthMarker::Len4 => Result::Ok(u64::from(reader.read_u32()?)),
            LengthMarker::Len8 => reader.read_u64(),
            _ => LqError::err_static("Invalid length for unsigned integer type"),
        }
    }
}

impl Serializer for TUInt {
    type Item = u64;

    fn serialize<W: BinaryWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        let item_deref = *item;
        match item_deref {
            0 => writer.write_header_u8(TYPE_UINT, 0),
            n if n <= u64::from(std::u8::MAX) => {
                writer.write_header_u8(TYPE_UINT, 1)?;
                writer.write_u8(item_deref as u8)
            }
            n if n <= u64::from(std::u16::MAX) => {
                writer.write_header_u8(TYPE_UINT, 2)?;
                writer.write_u16(item_deref as u16)
            }
            n if n <= u64::from(std::u32::MAX) => {
                writer.write_header_u8(TYPE_UINT, 4)?;
                writer.write_u32(item_deref as u32)
            }
            _ => {
                writer.write_header_u8(TYPE_UINT, 8)?;
                writer.write_u64(item_deref)
            }
        }
    }
}


impl<'a> DeSerializer<'a> for TUIntU8 {
    type Item = u8;

    fn de_serialize<R: BinaryReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let int = TUInt::de_serialize(reader)?;
        if int>std::u8::MAX as u64 {
            return LqError::err_new(format!("Value is not within the u8 integer range (0-255). Value is {:?}.",
            int));
        }
        Result::Ok(int as u8)
    }
}

impl Serializer for TUIntU8 {
    type Item = u8;

    fn serialize<W: BinaryWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        TUInt::serialize(writer, &(*item as u64))
    }
}
