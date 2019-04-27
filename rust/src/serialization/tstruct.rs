use crate::serialization::core::SkipMore;
use crate::serialization::type_ids::TYPE_STRUCT_0;
use crate::serialization::type_ids::TYPE_STRUCT_1;
use crate::serialization::type_ids::TYPE_STRUCT_2;
use crate::serialization::type_ids::TYPE_STRUCT_3;
use crate::serialization::type_ids::TYPE_STRUCT_U16;
use crate::serialization::type_ids::TYPE_STRUCT_U8;

use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::LqError;
use crate::serialization::core::Serializer;

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
    pub fn begin(&self, wanted_number_of_fields: usize) -> Result<StructRead, LqError> {
        if wanted_number_of_fields < self.number_of_fields {
            LqError::err_new(format!(
                "Expecting to have a struct with at least {:?} fields; 
            have {:?} fields.",
                wanted_number_of_fields, self.number_of_fields
            ))
        } else {
            Result::Ok(StructRead {
                actual_number_of_fields: self.number_of_fields,
                wanted_number_of_fields,
            })
        }
    }
}

pub struct StructRead {
    actual_number_of_fields: usize,
    wanted_number_of_fields: usize,
}

impl StructRead {
    pub fn finish<'a, T: BinaryReader<'a>>(self, reader: &mut T) -> Result<(), LqError> {
        let fields_to_skip = self.actual_number_of_fields - self.wanted_number_of_fields;
        if fields_to_skip > 0 {
            for _ in 0..fields_to_skip {
                reader.skip()?;
            }   
            Result::Ok(())     
        } else {
            Result::Ok(())
        }
    }
}

impl<'a> DeSerializer<'a> for StructInfo {
    type Item = Self;

    fn de_serialize<Reader: BinaryReader<'a>>(reader: &mut Reader) -> Result<Self::Item, LqError> {
        let header = reader.read_header()?;
        let id = header.type_id();

        let value = match id {
            TYPE_STRUCT_0 => StructInfo::new(0),
            TYPE_STRUCT_1 => StructInfo::new(1),
            TYPE_STRUCT_2 => StructInfo::new(2),
            TYPE_STRUCT_3 => StructInfo::new(3),
            TYPE_STRUCT_U8 => {
                let number_of_fields = reader.read_u8()? as usize;
                if number_of_fields <= 3 {
                    return LqError::err_static(
                        "Structure incorrectly encoded: For short 
                    structures (<=3 fields) you have to encode the length in the type. 
                    This is required to keep the data canonical.",
                    );
                }
                StructInfo::new(number_of_fields)
            }
            TYPE_STRUCT_U16 => {
                let number_of_fields = reader.read_u16()?;
                if number_of_fields <= std::u8::MAX as u16 {
                    return LqError::err_static(
                        "Structure incorrectly encoded: For short 
                    structures (#fields <= u8) you have to encode the length using the u8 type. 
                    This is required to keep the data canonical.",
                    );
                }
                StructInfo::new(number_of_fields as usize)
            }
            _ => return LqError::err_static("Not a structure (invalid type)"),
        };
        Result::Ok(value)
    }

    fn skip<T: BinaryReader<'a>>(reader: &mut T) -> Result<SkipMore, LqError> {
        let struct_info = Self::de_serialize(reader)?;
        Result::Ok(SkipMore::new(struct_info.number_of_fields()))
    }
}

impl<'a> Serializer for StructInfo {
    type Item = Self;

    fn serialize<'b, T: BinaryWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        let number_of_fields = item.number_of_fields;
        match number_of_fields {
            0 => writer.write_header_u8(TYPE_STRUCT_0, 0)?,
            1 => writer.write_header_u8(TYPE_STRUCT_1, 0)?,
            2 => writer.write_header_u8(TYPE_STRUCT_2, 0)?,
            3 => writer.write_header_u8(TYPE_STRUCT_3, 0)?,
            _ => {
                if number_of_fields <= std::u8::MAX as usize {
                    writer.write_header_u8(TYPE_STRUCT_U8, 1)?;
                    writer.write_u8(number_of_fields as u8)?;
                } else if number_of_fields <= std::u16::MAX as usize {
                    writer.write_header_u8(TYPE_STRUCT_U8, 2)?;
                    writer.write_u16(number_of_fields as u16)?;
                } else {
                    return LqError::err_static("Stucture can contain at max 2^16-1 fields.");
                }
            }
        };
        Result::Ok(())
    }
}
