use crate::serialization::type_ids::TYPE_STRUCT;

use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::LqError;
use crate::serialization::core::ContainerHeader;
use crate::serialization::core::Serializer;

pub struct StructData {
    number_of_fields: usize,
}

impl StructData {
    pub fn new(number_of_fields: usize) -> Self {
        StructData { number_of_fields }
    }

    pub fn number_of_fields(&self) -> usize {
        self.number_of_fields
    }

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

impl<'a> DeSerializer<'a> for StructData {
    type Item = Self;

    fn de_serialize<Reader: BinaryReader<'a>>(reader: &mut Reader) -> Result<Self::Item, LqError> {
        let header = reader.read_header()?;
        let container_info = reader.read_header_container(header)?;
        if header.type_id()!=TYPE_STRUCT {
            return LqError::err_static("Not a struct type");
        }
        if container_info.self_length()!=0 {
            return LqError::err_static("Invalid encoding; length of struct must be 0.");
        }
        Result::Ok(Self::Item{ number_of_fields : container_info.number_of_items() })
    }   
}

impl<'a> Serializer for StructData {
    type Item = Self;

    fn serialize<'b, T: BinaryWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        let number_of_fields = item.number_of_fields;
        let header = ContainerHeader::new(number_of_fields, 0);
        writer.write_container_header(TYPE_STRUCT,header )
    }
}
