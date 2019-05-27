use crate::major_types::TYPE_ENUM_0;
use crate::major_types::TYPE_ENUM_1;
use crate::major_types::TYPE_ENUM_2;
use crate::major_types::TYPE_ENUM_3;
use crate::major_types::TYPE_ENUM_N;

use crate::core::ContentDescription;
use crate::core::DeSerializer;
use crate::core::LqReader;
use crate::core::LqWriter;
use crate::core::Serializer;
use liquesco_common::error::LqError;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct EnumHeader {
    ordinal: u32,
    number_of_values: u32,
}

impl EnumHeader {
    pub fn new(ordinal: u32, number_of_values: u32) -> Self {
        EnumHeader {
            ordinal,
            number_of_values,
        }
    }

    /// Usually enums have 0 to 1 embedded values. But we also allow more values: this can be used
    /// for schema evolution - e.g. you can add additional values in the future.
    pub fn number_of_values(&self) -> u32 {
        self.number_of_values
    }

    pub fn ordinal(&self) -> u32 {
        self.ordinal
    }
}

impl<'a> DeSerializer<'a> for EnumHeader {
    type Item = Self;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let type_header = reader.read_type_header()?;
        let content_description = reader.read_content_description_given_type_header(type_header)?;
        let major_type = type_header.major_type();
        let self_length = content_description.self_length();

        // first we read the ordinal
        let ordinal: u32 = match major_type {
            TYPE_ENUM_N => match self_length {
                1 => u32::from(reader.read_u8()?),
                2 => u32::from(reader.read_u16()?),
                4 => reader.read_u32()?,
                _ => {
                    return LqError::err_static("Invalid enum self length.");
                }
            },
            _ => {
                // length has to be 0 here
                if self_length != 0 {
                    return LqError::err_new(format!(
                        "Expecting to have a self length of 0; have {:?}.",
                        self_length
                    ));
                }
                match major_type {
                    TYPE_ENUM_0 => 0,
                    TYPE_ENUM_1 => 1,
                    TYPE_ENUM_2 => 2,
                    TYPE_ENUM_3 => 3,
                    n => return LqError::err_new(format!("Not an enum type; \
                    wrong major type. Need one of the enum major types. Have {:?}.", n)),
                }
            }
        };

        Result::Ok(Self {
            ordinal,
            number_of_values: content_description.number_of_embedded_values(),
        })
    }
}

impl<'a> Serializer for EnumHeader {
    type Item = Self;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        let ordinal: u32 = item.ordinal;
        let major_type = match ordinal {
            0 => TYPE_ENUM_0,
            1 => TYPE_ENUM_1,
            2 => TYPE_ENUM_2,
            3 => TYPE_ENUM_3,
            _ => TYPE_ENUM_N,
        };

        let self_len = if major_type == TYPE_ENUM_N {
            match ordinal {
                n if n <= u32::from(std::u8::MAX) => 1,
                n if n <= u32::from(std::u16::MAX) => 2,
                _ => 4,
            }
        } else {
            0
        };

        // write header
        let mut content_description = ContentDescription::default();
        content_description.set_self_length(self_len);
        content_description.set_number_of_embedded_values(item.number_of_values);
        writer.write_content_description(major_type, &content_description)?;

        // depending on the ordinal we also have to write the ordinal
        if major_type == TYPE_ENUM_N {
            match ordinal {
                n if n <= u32::from(std::u8::MAX) => writer.write_u8(ordinal as u8),
                n if n <= u32::from(std::u16::MAX) => writer.write_u16(ordinal as u16),
                _ => writer.write_u32(ordinal),
            }?;
        }

        Result::Ok(())
    }
}
