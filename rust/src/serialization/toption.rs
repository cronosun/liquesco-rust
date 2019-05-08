use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::ContentDescription;
use crate::serialization::core::DeSerializer;
use crate::common::error::LqError;
use crate::serialization::core::Serializer;
use crate::serialization::major_types::TYPE_OPTION;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Presence {
    Present,
    Absent,
}

impl<'a> DeSerializer<'a> for Presence {
    type Item = Self;

    fn de_serialize<R: BinaryReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let type_header = reader.read_type_header()?;
        let content_description = reader.read_content_description_given_type_header(type_header)?;

        if type_header.major_type() != TYPE_OPTION {
            return LqError::err_static("Given type is not the option type");
        }
        if content_description.self_length()!=0 {
            return LqError::err_new(format!("Option types must have a self length of 0 (this value has a self 
            length of {:?})", content_description.self_length()));
        }

        match content_description.number_of_embedded_values() {
            0 => Result::Ok(Presence::Absent),
            1 => Result::Ok(Presence::Present),
            n => LqError::err_new(format!("Invalid option type (option types need to have 0 or 1 
            embedded item(s)). This value has {:?} embedded items.", n)),
        }
    }
}

impl Serializer for Presence {
    type Item = Self;

    fn serialize<W: BinaryWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        match item {
            Presence::Present => {
                writer.write_content_description(TYPE_OPTION, &ContentDescription::new_number_of_embedded_values(1))                
            }
            Presence::Absent => {
                writer.write_content_description(TYPE_OPTION, &ContentDescription::default())                
            },
        }
    }
}
