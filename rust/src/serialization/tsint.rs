use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::LengthMarker;
use crate::serialization::core::LqError;
use crate::serialization::core::Serializer;
use crate::serialization::type_ids::TYPE_SINT;

pub struct TSInt;

impl<'a> DeSerializer<'a> for TSInt {
    type Item = i64;

    fn de_serialize<Reader: BinaryReader<'a>>(reader: &mut Reader) -> Result<Self::Item, LqError> {
        let header = reader.read_header()?;
        if header.type_id() != TYPE_SINT {
            return LqError::err_static("Given type is not an unsigned integer type");
        }
        match header.length_marker() {
            LengthMarker::Len0 => Result::Ok(i64::from(reader.read_i8()?)),
            LengthMarker::Len2 => Result::Ok(i64::from(reader.read_i16()?)),
            LengthMarker::Len4 => Result::Ok(i64::from(reader.read_i32()?)),
            LengthMarker::Len8 => reader.read_i64(),
            _ => LqError::err_static("Invalid length for unsigned integer type"),
        }
    }
}

impl Serializer for TSInt {
    type Item = i64;

    fn serialize<T: BinaryWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        let deref_item = *item;
        match deref_item {
            n if n >= i64::from(std::i8::MIN) && n <= i64::from(std::i8::MAX) => {
                writer.write_header_u8(TYPE_SINT, 1)?;
                writer.write_i8(deref_item as i8)
            }
            n if n >= i64::from(std::i16::MIN)  && n <= i64::from(std::i16::MAX)=> {
                writer.write_header_u8(TYPE_SINT, 2)?;
                writer.write_i16(deref_item as i16)
            }
            n if n >= i64::from(std::i32::MIN) && n <= i64::from(std::i32::MAX) => {
                writer.write_header_u8(TYPE_SINT, 4)?;
                writer.write_i32(deref_item as i32)
            }
            _ => {
                writer.write_header_u8(TYPE_SINT, 8)?;
                writer.write_i64(*item)
            }
        }
    }
}
