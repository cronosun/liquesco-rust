use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::LqError;
use crate::serialization::core::SkipMore;
use crate::serialization::core::TypeId;
use crate::serialization::core::TypeReader;
use crate::serialization::core::TypeWriter;
use crate::serialization::types::TYPE_OPTION_ABSENT;
use crate::serialization::types::TYPE_OPTION_PRESENT;

pub struct TOption;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Presence {
    Present,
    Absent,
}

impl<'a> TypeReader<'a> for TOption {
    type Item = Presence;

    fn read<Reader: BinaryReader<'a>>(id: TypeId, _: &mut Reader) -> Result<Self::Item, LqError> {
        match id {
            TYPE_OPTION_PRESENT => Result::Ok(Presence::Present),
            TYPE_OPTION_ABSENT => Result::Ok(Presence::Absent),
            _ => LqError::err_static("Type is not an option type"),
        }
    }

    fn skip<Reader: BinaryReader<'a>>(
        id: TypeId,
        reader: &mut Reader,
    ) -> Result<SkipMore, LqError> {
        match Self::read(id, reader)? {
            Presence::Present => Result::Ok(SkipMore::new(1)),
            Presence::Absent => Result::Ok(SkipMore::new(0)),
        }
    }
}

impl TypeWriter for TOption {
    type Item = Presence;

    fn write<'b, Writer: BinaryWriter<'b> + 'b>(
        writer: Writer,
        item: &Self::Item,
    ) -> Result<(), LqError> {
        match item {
            Presence::Present => {
                writer.begin(TYPE_OPTION_PRESENT)?;
                Result::Ok(())
            }
            Presence::Absent => {
                writer.begin(TYPE_OPTION_ABSENT)?;
                Result::Ok(())
            }
        }
    }
}
