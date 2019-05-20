use crate::common::error::LqError;
use crate::serialization::core::ContentDescription;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::LqReader;
use crate::serialization::core::LqWriter;
use crate::serialization::core::Serializer;
use crate::serialization::major_types::TYPE_UINT;
use std::convert::TryFrom;

pub struct UInt64;

impl<'a> DeSerializer<'a> for UInt64 {
    type Item = u64;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let type_header = reader.read_type_header()?;
        let content_description = reader.read_content_description_given_type_header(type_header)?;

        if type_header.major_type() != TYPE_UINT {
            return LqError::err_new(format!(
                "Given type is not an unsigned integer type. \
                 Major type is {:?}.",
                type_header.major_type()
            ));
        }
        if content_description.number_of_embedded_values() != 0 {
            return LqError::err_static("Integer types must not contain embedded values.");
        }

        match content_description.self_length() {
            0 => Result::Ok(0),
            1 => Result::Ok(u64::from(reader.read_u8()?)),
            2 => Result::Ok(u64::from(reader.read_u16()?)),
            4 => Result::Ok(u64::from(reader.read_u32()?)),
            8 => reader.read_u64(),
            _ => LqError::err_static("Invalid length for unsigned integer type"),
        }
    }
}

impl Serializer for UInt64 {
    type Item = u64;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        let deref_item = *item;
        match deref_item {
            0 => writer.write_content_description(TYPE_UINT, &ContentDescription::default()),
            n if n >= u64::from(std::u8::MIN) && n <= u64::from(std::u8::MAX) => {
                writer.write_content_description(
                    TYPE_UINT,
                    &ContentDescription::new_self_length(1),
                )?;
                writer.write_u8(deref_item as u8)
            }
            n if n >= u64::from(std::u16::MIN) && n <= u64::from(std::u16::MAX) => {
                writer.write_content_description(
                    TYPE_UINT,
                    &ContentDescription::new_self_length(2),
                )?;
                writer.write_u16(deref_item as u16)
            }
            n if n >= u64::from(std::u32::MIN) && n <= u64::from(std::u32::MAX) => {
                writer.write_content_description(
                    TYPE_UINT,
                    &ContentDescription::new_self_length(4),
                )?;
                writer.write_u32(deref_item as u32)
            }
            _ => {
                writer.write_content_description(
                    TYPE_UINT,
                    &ContentDescription::new_self_length(8),
                )?;
                writer.write_u64(*item)
            }
        }
    }
}

pub struct UInt8;

impl<'a> DeSerializer<'a> for UInt8 {
    type Item = u8;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let value = UInt64::de_serialize(reader)?;
        Ok(Self::Item::try_from(value)?)
    }
}

impl Serializer for UInt8 {
    type Item = u8;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        UInt64::serialize(writer, &(*item as u64))
    }
}

pub struct UInt16;

impl<'a> DeSerializer<'a> for UInt16 {
    type Item = u16;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let value = UInt64::de_serialize(reader)?;
        Ok(Self::Item::try_from(value)?)
    }
}

impl Serializer for UInt16 {
    type Item = u16;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        UInt64::serialize(writer, &(*item as u64))
    }
}

pub struct UInt32;

impl<'a> DeSerializer<'a> for UInt32 {
    type Item = u32;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let value = UInt64::de_serialize(reader)?;
        Ok(Self::Item::try_from(value)?)
    }
}

impl Serializer for UInt32 {
    type Item = u32;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        UInt64::serialize(writer, &(*item as u64))
    }
}
