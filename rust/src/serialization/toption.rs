use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::LengthMarker;
use crate::serialization::core::LqError;
use crate::serialization::core::Serializer;
use crate::serialization::core::ContainerHeader;
use crate::serialization::type_ids::TYPE_OPTION;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Presence {
    Present,
    Absent,
}

impl<'a> DeSerializer<'a> for Presence {
    type Item = Self;

    fn de_serialize<Reader: BinaryReader<'a>>(reader: &mut Reader) -> Result<Self::Item, LqError> {
        let header = reader.read_header()?;
        if header.type_id() != TYPE_OPTION {
            return LqError::err_static("Given type is not the option type");
        }
        match header.length_marker() {
            LengthMarker::Len0 => Result::Ok(Presence::Absent),
            LengthMarker::ConainerOneEmpty => Result::Ok(Presence::Present),
            _ => return LqError::err_static("Invalid option type"),
        }
    }
}

impl Serializer for Presence {
    type Item = Self;

    fn serialize<'b, T: BinaryWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        match item {
            Presence::Present => {
                writer.write_container_header(TYPE_OPTION, ContainerHeader::new(1, 0))
            }
            Presence::Absent => writer.write_header_u8(TYPE_OPTION, 0),
        }
    }
}
