use crate::serialization::types::TYPE_OPTION_ABSENT;
use crate::serialization::types::TYPE_OPTION_PRESENT;
use crate::serialization::core::HeaderWriter;
use crate::serialization::core::Type;
use crate::serialization::core::LqError;
use crate::serialization::core::ReadResult;
use crate::serialization::core::TypeId;

pub struct TOption;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Option {
    Present,
    Absent
}

impl<'a> Type<'a> for TOption {
    type ReadItem = Option;
    type WriteItem = Option;

    fn read(id: TypeId, _: &[u8]) -> Result<ReadResult<Self::ReadItem>, LqError> {
        match id {
            TYPE_OPTION_PRESENT => ReadResult::new_ok(0, Option::Present),
            TYPE_OPTION_ABSENT => ReadResult::new_ok(0, Option::Absent),
            _ => LqError::err_static("Type is not an option type"),
        }
    }

    fn write<'b, Writer: HeaderWriter<'b> + 'b>(
        writer: Writer,
        item: &Self::WriteItem,
    ) -> Result<(), LqError> {
        match item {
            Option::Present => {
                writer.type_id(TYPE_OPTION_PRESENT);
                Result::Ok(())
            }
            Option::Absent => {
                writer.type_id(TYPE_OPTION_ABSENT);
                Result::Ok(())
            }
        }
    }
}
