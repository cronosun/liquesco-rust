use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::LqError;
use crate::serialization::core::TypeId;
use crate::serialization::core::TypeReader;
use crate::serialization::core::TypeWriter;
use crate::serialization::types::TYPE_OPTION_ABSENT;
use crate::serialization::types::TYPE_OPTION_PRESENT;

pub struct TOption;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Option {
    Present,
    Absent,
}

impl<'a> TypeReader<'a> for TOption {
    type Item = Option;

    fn read<Reader: BinaryReader>(id: TypeId, _: &'a mut Reader) -> Result<Self::Item, LqError> {
        match id {
            TYPE_OPTION_PRESENT => Result::Ok(Option::Present),
            TYPE_OPTION_ABSENT => Result::Ok(Option::Absent),
            _ => LqError::err_static("Type is not an option type"),
        }
    }
}

impl TypeWriter for TOption {
    type Item = Option;

    fn write<'b, Writer: BinaryWriter<'b> + 'b>(
        writer: Writer,
        item: &Self::Item,
    ) -> Result<(), LqError> {
        match item {
            Option::Present => {
                writer.begin(TYPE_OPTION_PRESENT)?;
                Result::Ok(())
            }
            Option::Absent => {
                writer.begin(TYPE_OPTION_ABSENT)?;
                Result::Ok(())
            }
        }
    }
}
