use crate::core::ContentDescription;
use crate::core::DeSerializer;
use crate::core::LqReader;
use crate::core::LqWriter;
use crate::core::Serializer;
use crate::major_types::TYPE_UINT;
use liquesco_common::error::LqError;
use std::convert::TryFrom;

/// 128 bit unsigned integer.
pub struct UInt128;

impl<'a> DeSerializer<'a> for UInt128 {
    type Item = u128;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let type_header = reader.read_header_byte()?;
        let content_description = reader.read_content_description_given_header_byte(type_header)?;

        if type_header.major_type() != TYPE_UINT {
            return LqError::err_new(format!(
                "Given type is not an unsigned integer type. \
                 Major type is {:?}.",
                type_header.major_type()
            ));
        }
        if content_description.number_of_embedded_items() != 0 {
            return LqError::err_new("Integer types must not contain embedded values.");
        }

        match content_description.self_length() {
            0 => Result::Ok(0),
            1 => Result::Ok(u128::from(reader.read_u8()?)),
            2 => Result::Ok(u128::from(reader.read_u16()?)),
            4 => Result::Ok(u128::from(reader.read_u32()?)),
            8 => Result::Ok(u128::from(reader.read_u64()?)),
            16 => Result::Ok(reader.read_u128()?),
            _ => LqError::err_new("Invalid length for unsigned integer type"),
        }
    }
}

impl Serializer for UInt128 {
    type Item = u128;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        let deref_item = *item;
        match deref_item {
            0 => writer.write_content_description(TYPE_UINT, &ContentDescription::default()),
            n if n >= u128::from(std::u8::MIN) && n <= u128::from(std::u8::MAX) => {
                writer.write_content_description(
                    TYPE_UINT,
                    &ContentDescription::new_self_length(1),
                )?;
                writer.write_u8(deref_item as u8)
            }
            n if n >= u128::from(std::u16::MIN) && n <= u128::from(std::u16::MAX) => {
                writer.write_content_description(
                    TYPE_UINT,
                    &ContentDescription::new_self_length(2),
                )?;
                writer.write_u16(deref_item as u16)
            }
            n if n >= u128::from(std::u32::MIN) && n <= u128::from(std::u32::MAX) => {
                writer.write_content_description(
                    TYPE_UINT,
                    &ContentDescription::new_self_length(4),
                )?;
                writer.write_u32(deref_item as u32)
            }
            n if n >= u128::from(std::u64::MIN) && n <= u128::from(std::u64::MAX) => {
                writer.write_content_description(
                    TYPE_UINT,
                    &ContentDescription::new_self_length(8),
                )?;
                writer.write_u64(deref_item as u64)
            }
            _ => {
                writer.write_content_description(
                    TYPE_UINT,
                    &ContentDescription::new_self_length(16),
                )?;
                writer.write_u128(*item)
            }
        }
    }
}

/// 64 bit unsigned integer.
pub struct UInt64;

impl<'a> DeSerializer<'a> for UInt64 {
    type Item = u64;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let value = UInt128::de_serialize(reader)?;
        Ok(Self::Item::try_from(value)?)
    }
}

impl Serializer for UInt64 {
    type Item = u64;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        UInt128::serialize(writer, &u128::from(*item))
    }
}

pub struct UInt8;

impl<'a> DeSerializer<'a> for UInt8 {
    type Item = u8;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let value = UInt128::de_serialize(reader)?;
        Ok(Self::Item::try_from(value)?)
    }
}

impl Serializer for UInt8 {
    type Item = u8;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        UInt128::serialize(writer, &u128::from(*item))
    }
}

pub struct UInt16;

impl<'a> DeSerializer<'a> for UInt16 {
    type Item = u16;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let value = UInt128::de_serialize(reader)?;
        Ok(Self::Item::try_from(value)?)
    }
}

impl Serializer for UInt16 {
    type Item = u16;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        UInt128::serialize(writer, &u128::from(*item))
    }
}

pub struct UInt32;

impl<'a> DeSerializer<'a> for UInt32 {
    type Item = u32;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let value = UInt128::de_serialize(reader)?;
        Ok(Self::Item::try_from(value)?)
    }
}

impl Serializer for UInt32 {
    type Item = u32;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        UInt128::serialize(writer, &u128::from(*item))
    }
}
