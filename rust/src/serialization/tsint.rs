use crate::common::error::LqError;
use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::ContentDescription;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::Serializer;
use crate::serialization::type_ids::TYPE_SINT;

pub struct TSInt;

impl<'a> DeSerializer<'a> for TSInt {
    type Item = i64;

    fn de_serialize<R: BinaryReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let type_header = reader.read_type_header()?;
        let content_description = reader.read_content_description_given_type_header(type_header)?;

        if type_header.major_type() != TYPE_SINT {
            return LqError::err_static("Given type is not an unsigned integer type");
        }
        if content_description.number_of_embedded_values() != 0 {
            return LqError::err_static("Integer types must not contain embedded values.");
        }

        match content_description.self_length() {
            0 => Result::Ok(0),
            1 => Result::Ok(i64::from(reader.read_i8()?)),
            2 => Result::Ok(i64::from(reader.read_i16()?)),
            4 => Result::Ok(i64::from(reader.read_i32()?)),
            8 => reader.read_i64(),
            _ => LqError::err_static("Invalid length for signed integer type"),
        }
    }
}

impl Serializer for TSInt {
    type Item = i64;

    fn serialize<W: BinaryWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        let deref_item = *item;
        match deref_item {
            0 => writer.write_content_description(TYPE_SINT, &ContentDescription::default()),
            n if n >= i64::from(std::i8::MIN) && n <= i64::from(std::i8::MAX) => {
                writer.write_content_description(
                    TYPE_SINT,
                    &ContentDescription::new_self_length(1),
                )?;
                writer.write_i8(deref_item as i8)
            }
            n if n >= i64::from(std::i16::MIN) && n <= i64::from(std::i16::MAX) => {
                writer.write_content_description(
                    TYPE_SINT,
                    &ContentDescription::new_self_length(2),
                )?;
                writer.write_i16(deref_item as i16)
            }
            n if n >= i64::from(std::i32::MIN) && n <= i64::from(std::i32::MAX) => {
                writer.write_content_description(
                    TYPE_SINT,
                    &ContentDescription::new_self_length(4),
                )?;
                writer.write_i32(deref_item as i32)
            }
            _ => {
                writer.write_content_description(
                    TYPE_SINT,
                    &ContentDescription::new_self_length(8),
                )?;
                writer.write_i64(*item)
            }
        }
    }
}
