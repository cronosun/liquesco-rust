use crate::serialization::type_ids::TYPE_ENUM_0;
use crate::serialization::type_ids::TYPE_ENUM_1;
use crate::serialization::type_ids::TYPE_ENUM_2;
use crate::serialization::type_ids::TYPE_ENUM_N;
use crate::serialization::util::try_from_int_result;
use std::convert::TryFrom;

use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::ContainerHeader;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::LengthMarker;
use crate::serialization::core::LqError;
use crate::serialization::core::Serializer;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct EnumData {
    ordinal: usize,
    has_value: bool,
}

impl EnumData {
    pub fn new(ordinal: usize) -> Self {
        Self {
            ordinal,
            has_value: false,
        }
    }

    pub fn new_with_value(ordinal: usize) -> Self {
        Self {
            ordinal,
            has_value: true,
        }
    }

    pub fn has_value(&self) -> bool {
        self.has_value
    }

    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
}

impl<'a> DeSerializer<'a> for EnumData {
    type Item = Self;

    fn de_serialize<Reader: BinaryReader<'a>>(reader: &mut Reader) -> Result<Self::Item, LqError> {
        let header = reader.read_header()?;

        if header.type_id() == TYPE_ENUM_N {
            // container type
            let container = reader.read_header_container(header)?;
            let has_value = match container.number_of_items() {
                0 => false,
                1 => true,
                _ => {
                    return LqError::err_static(
                        "Enum can contain at max one item. Has more than one embedded item.",
                    );
                }
            };
            // read ordinal
            let self_len = container.self_length();
            let ordinal = match self_len {
                1 => reader.read_u8()? as usize,
                2 => reader.read_u16()? as usize,
                4 => try_from_int_result(usize::try_from(reader.read_u32()?))?,
                _ => {
                    return LqError::err_static("Invalid enum self length.");
                }
            };
            Result::Ok(Self::Item { ordinal, has_value })
        } else {
            // no a container type
            let ordinal = match header.type_id() {
                TYPE_ENUM_0 => 0,
                TYPE_ENUM_1 => 1,
                TYPE_ENUM_2 => 2,
                _ => return LqError::err_static("Not an enum type."),
            };
            let has_value = match header.length_marker() {
                LengthMarker::ConainerOneEmpty => true,
                LengthMarker::Len0 => false,
                _ => return LqError::err_static("Invalid length marker for enum."),
            };
            Result::Ok(Self::Item { ordinal, has_value })
        }
    }
}

impl<'a> Serializer for EnumData {
    type Item = Self;

    fn serialize<T: BinaryWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        let ordinal = item.ordinal;
        if ordinal > 2 {
            // need a container
            let number_of_items = if item.has_value { 1 } else { 0 };
            let self_len = match ordinal {
                n if n <= std::u8::MAX as usize => 1,
                n if n <= std::u16::MAX as usize => 2,
                n if n <= std::u32::MAX as usize => 4,
                _ => {
                    return LqError::err_static(
                        "Enum ordinal is too large; supports at max 2^32-1.",
                    )
                }
            };
            let container_header = ContainerHeader::new(number_of_items, self_len);
            writer.write_container_header(TYPE_ENUM_N, container_header)?;

            // write ordinal
            match ordinal {
                n if n <= std::u8::MAX as usize => writer.write_u8(ordinal as u8),
                n if n <= std::u16::MAX as usize => writer.write_u16(ordinal as u16),
                n if n <= std::u32::MAX as usize => writer.write_u32(ordinal as u32),
                _ => {
                    return LqError::err_static(
                        "Enum ordinal is too large; supports at max 2^32-1.",
                    )
                }
            }?;
            Result::Ok(())
        } else {
            let type_id = match ordinal {
                0 => TYPE_ENUM_0,
                1 => TYPE_ENUM_1,
                2 => TYPE_ENUM_2,
                _ => return LqError::err_static("Impelentation error"),
            };
            if item.has_value {
                let container_header = ContainerHeader::new(1, 0);
                writer.write_container_header(type_id, container_header)
            } else {
                writer.write_header_u8(type_id, 0)
            }
        }
    }
}
