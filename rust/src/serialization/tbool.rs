use crate::common::error::LqError;
use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::ContentDescription;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::Serializer;
use crate::serialization::major_types::TYPE_BOOL_FALSE;
use crate::serialization::major_types::TYPE_BOOL_TRUE;

pub struct TBool;

impl<'a> DeSerializer<'a> for TBool {
    type Item = bool;

    fn de_serialize<R: BinaryReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let major_type = reader.read_expect_content_description(0, 0)?;
        match major_type {
            TYPE_BOOL_TRUE => Result::Ok(true),
            TYPE_BOOL_FALSE => Result::Ok(false),
            _ => LqError::err_static("Type is not a boolean"),
        }
    }
}

impl Serializer for TBool {
    type Item = bool;

    fn serialize<W: BinaryWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        match item {
            true => {
                writer.write_content_description(TYPE_BOOL_TRUE, &ContentDescription::default())
            }
            false => {
                writer.write_content_description(TYPE_BOOL_FALSE, &ContentDescription::default())
            }
        }
    }
}

// TODO: Change bool to TBool (to be consistent with rest)

impl<'a> DeSerializer<'a> for bool {
    type Item = Self;

    fn de_serialize<R: BinaryReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let major_type = reader.read_expect_content_description(0, 0)?;
        match major_type {
            TYPE_BOOL_TRUE => Result::Ok(true),
            TYPE_BOOL_FALSE => Result::Ok(false),
            _ => LqError::err_static("Type is not a boolean"),
        }
    }
}

impl Serializer for bool {
    type Item = Self;

    fn serialize<W: BinaryWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        match item {
            true => {
                writer.write_content_description(TYPE_BOOL_TRUE, &ContentDescription::default())
            }
            false => {
                writer.write_content_description(TYPE_BOOL_FALSE, &ContentDescription::default())
            }
        }
    }
}
