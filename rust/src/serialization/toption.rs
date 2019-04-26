use crate::serialization::core::SkipMore;
use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::LqError;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::Serializer;
use crate::serialization::type_ids::TYPE_OPTION_ABSENT;
use crate::serialization::type_ids::TYPE_OPTION_PRESENT;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Presence {
    Present,
    Absent,
}

impl<'a> DeSerializer<'a> for Presence {
    type Item = Self;

    fn de_serialize<Reader: BinaryReader<'a>>(reader: &mut Reader) -> Result<Self::Item, LqError> {
        let id = reader.type_id()?;
        match id {
            TYPE_OPTION_PRESENT => Result::Ok(Presence::Present),
            TYPE_OPTION_ABSENT => Result::Ok(Presence::Absent),
            _ => LqError::err_static("Type is not an option type"),
        }
    }

    fn skip<T: BinaryReader<'a>>(reader: &mut T) -> Result<SkipMore, LqError> {
        let presence = Self::de_serialize(reader)?;
        match presence {
            Presence::Present => Result::Ok(SkipMore::new(1)),
            Presence::Absent => Result::Ok(SkipMore::new(0))
        }        
    }   
}

impl Serializer for Presence {
    type Item = Self;

    fn serialize<'b, T: BinaryWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        match item {
            Presence::Present => {
                writer.type_id(TYPE_OPTION_PRESENT)?;
                Result::Ok(())
            }
            Presence::Absent => {
                writer.type_id(TYPE_OPTION_ABSENT)?;
                Result::Ok(())
            }
        }
    }
}
