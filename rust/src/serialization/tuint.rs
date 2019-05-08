use crate::common::internal_utils::try_from_int_result;
use crate::common::error::LqError;
use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::ContentDescription;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::Serializer;
use crate::serialization::major_types::TYPE_UINT;
use std::convert::TryFrom;

pub struct TUInt;

impl<'a> DeSerializer<'a> for TUInt {
    type Item = u64;

    fn de_serialize<R: BinaryReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let type_header = reader.read_type_header()?;
        let content_description = reader.read_content_description_given_type_header(type_header)?;

        if type_header.major_type() != TYPE_UINT {
            return LqError::err_static("Given type is not an unsigned integer type");
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

impl Serializer for TUInt {
    type Item = u64;

    fn serialize<W: BinaryWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
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

pub struct TUInt8;

impl<'a> DeSerializer<'a> for TUInt8 {
    type Item = u8;

    fn de_serialize<R: BinaryReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let value = TUInt::de_serialize(reader)?;
        try_from_int_result(Self::Item::try_from(value))
    }
}

impl Serializer for TUInt8 {
    type Item = u8;

    fn serialize<W: BinaryWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        TUInt::serialize(writer, &(*item as u64))
    }
}

pub struct TUInt16;

impl<'a> DeSerializer<'a> for TUInt16 {
    type Item = u16;

    fn de_serialize<R: BinaryReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let value = TUInt::de_serialize(reader)?;
        try_from_int_result(Self::Item::try_from(value))
    }
}

impl Serializer for TUInt16 {
    type Item = u16;

    fn serialize<W: BinaryWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        TUInt::serialize(writer, &(*item as u64))
    }
}

pub struct TUInt32;

impl<'a> DeSerializer<'a> for TUInt32 {
    type Item = u32;

    fn de_serialize<R: BinaryReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let value = TUInt::de_serialize(reader)?;
        try_from_int_result(Self::Item::try_from(value))
    }
}

impl Serializer for TUInt32 {
    type Item = u32;

    fn serialize<W: BinaryWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        TUInt::serialize(writer, &(*item as u64))
    }
}