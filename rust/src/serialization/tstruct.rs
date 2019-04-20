use crate::serialization::types::TYPE_STRUCT_0;
use crate::serialization::types::TYPE_STRUCT_1;
use crate::serialization::types::TYPE_STRUCT_2;
use crate::serialization::types::TYPE_STRUCT_3;
use crate::serialization::types::TYPE_STRUCT_4;
use crate::serialization::types::TYPE_STRUCT_5;
use crate::serialization::types::TYPE_STRUCT_6;
use crate::serialization::types::TYPE_STRUCT_U16;
use crate::serialization::types::TYPE_STRUCT_U8;
use crate::serialization::util::io_result;

use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::LqError;
use crate::serialization::core::TypeId;
use crate::serialization::core::TypeReader;
use crate::serialization::core::TypeWriter;

use byteorder::ByteOrder;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Write;

pub struct TStruct;

pub struct StructInfo {
    number_of_fields: usize,
}

impl StructInfo {
    pub fn new(number_of_fields: usize) -> Self {
        StructInfo { number_of_fields }
    }

    pub fn number_of_fields(&self) -> usize {
        self.number_of_fields
    }

    // TODO: Need to have assert_minimum and then skip the rest of the fields
    pub fn assert(&self, number_of_fields: usize) -> Result<(), LqError> {
        if self.number_of_fields != number_of_fields {
            LqError::err_new(format!(
                "Expecting to have a struct with {:?}; 
            have {:?} fields.",
                number_of_fields, self.number_of_fields
            ))
        } else {
            Result::Ok(())
        }
    }
}

impl<'a> TypeReader<'a> for TStruct {
    type Item = StructInfo;

    fn read<Reader: BinaryReader<'a>>(
        id: TypeId,
        reader: &mut Reader,
    ) -> Result<Self::Item, LqError> {
        let value = match id {
            TYPE_STRUCT_0 => StructInfo::new(0),
            TYPE_STRUCT_1 => StructInfo::new(1),
            TYPE_STRUCT_2 => StructInfo::new(2),
            TYPE_STRUCT_3 => StructInfo::new(3),
            TYPE_STRUCT_4 => StructInfo::new(4),
            TYPE_STRUCT_5 => StructInfo::new(5),
            TYPE_STRUCT_6 => StructInfo::new(6),
            TYPE_STRUCT_U8 => {
                let number_of_fields = reader.read_u8()? as usize;
                if number_of_fields <= 6 {
                    return LqError::err_static(
                        "Structure incorrectly encoded: For short 
                    structures (<=6 fields) you have to encode the length in the type. 
                    This is required to keep the data canonical.",
                    );
                }
                StructInfo::new(number_of_fields)
            }
            TYPE_STRUCT_U16 => {
                let slice = reader.read_slice(2)?;
                let number_of_fields = LittleEndian::read_u16(slice) as usize;
                if number_of_fields <= std::u8::MAX as usize {
                    return LqError::err_static(
                        "Structure incorrectly encoded: For short 
                    structures (#fields <= u8) you have to encode the length using the u8 type. 
                    This is required to keep the data canonical.",
                    );
                }
                StructInfo::new(number_of_fields)
            }
            _ => return LqError::err_static("Not a structure (invalid type)"),
        };
        Result::Ok(value)
    }
}

impl<'a> TypeWriter for TStruct {
    type Item = StructInfo;

    fn write<'b, Writer: BinaryWriter<'b> + 'b>(
        writer: Writer,
        item: &Self::Item,
    ) -> Result<(), LqError> {
        let number_of_fields = item.number_of_fields;
        match number_of_fields {
            0 => writer.begin(TYPE_STRUCT_0)?,
            1 => writer.begin(TYPE_STRUCT_1)?,
            2 => writer.begin(TYPE_STRUCT_2)?,
            3 => writer.begin(TYPE_STRUCT_3)?,
            4 => writer.begin(TYPE_STRUCT_4)?,
            5 => writer.begin(TYPE_STRUCT_5)?,
            6 => writer.begin(TYPE_STRUCT_6)?,
            _ => {
                if number_of_fields <= std::u8::MAX as usize {
                    let body_writer = writer.begin(TYPE_STRUCT_U8)?;
                    io_result(body_writer.write(&[number_of_fields as u8]))?;
                    body_writer
                } else if number_of_fields <= std::u16::MAX as usize {
                    let body_writer = writer.begin(TYPE_STRUCT_U16)?;
                    io_result(body_writer.write_u16::<LittleEndian>(number_of_fields as u16))?;
                    body_writer
                } else {
                    return LqError::err_static("Stucture can contain at max 2^16-1 fields.");
                }
            }
        };
        Result::Ok(())
    }
}
