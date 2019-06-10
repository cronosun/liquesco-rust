use crate::core::ContentDescription;
use crate::core::DeSerializer;
use crate::core::LqReader;
use crate::core::LqWriter;
use crate::core::Serializer;
use crate::major_types::TYPE_SINT;
use liquesco_common::error::LqError;
use std::convert::TryFrom;

/// A 64 bit signed integer.
pub struct SInt64;

impl<'a> DeSerializer<'a> for SInt64 {
    type Item = i64;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let type_header = reader.read_header_byte()?;
        let content_description = reader.read_content_description_given_header_byte(type_header)?;

        if type_header.major_type() != TYPE_SINT {
            return LqError::err_new(format!(
                "Given type is not a signed integer type; \
                 major type is {:?}.",
                type_header.major_type()
            ));
        }
        if content_description.number_of_embedded_items() != 0 {
            return LqError::err_new("Integer types must not contain embedded values.");
        }

        match content_description.self_length() {
            0 => Result::Ok(0),
            1 => Result::Ok(i64::from(reader.read_i8()?)),
            2 => Result::Ok(i64::from(reader.read_i16()?)),
            4 => Result::Ok(i64::from(reader.read_i32()?)),
            8 => reader.read_i64(),
            _ => LqError::err_new("Invalid length for signed integer type"),
        }
    }
}

impl Serializer for SInt64 {
    type Item = i64;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
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

pub struct SInt8;

impl<'a> DeSerializer<'a> for SInt8 {
    type Item = i8;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let value = SInt64::de_serialize(reader)?;
        Ok(Self::Item::try_from(value)?)
    }
}

impl Serializer for SInt8 {
    type Item = i8;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        SInt64::serialize(writer, &i64::from(*item))
    }
}

pub struct SInt16;

impl<'a> DeSerializer<'a> for SInt16 {
    type Item = i16;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let value = SInt64::de_serialize(reader)?;
        Ok(Self::Item::try_from(value)?)
    }
}

impl Serializer for SInt16 {
    type Item = i16;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        SInt64::serialize(writer, &i64::from(*item))
    }
}

pub struct SInt32;

impl<'a> DeSerializer<'a> for SInt32 {
    type Item = i32;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let value = SInt64::de_serialize(reader)?;
        Ok(Self::Item::try_from(value)?)
    }
}

impl Serializer for SInt32 {
    type Item = i32;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        SInt64::serialize(writer, &i64::from(*item))
    }
}
