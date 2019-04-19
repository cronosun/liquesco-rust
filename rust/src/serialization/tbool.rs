use crate::serialization::core::BinaryReader;
use crate::serialization::types::TYPE_BOOL_FALSE;
use crate::serialization::types::TYPE_BOOL_TRUE;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::Type;
use crate::serialization::core::LqError;
use crate::serialization::core::TypeId;

pub struct TBool;

impl<'a> Type<'a> for TBool {
    type ReadItem = bool;
    type WriteItem = bool;

    fn read<Reader : BinaryReader>(
        id: TypeId, _: &'a mut Reader) -> Result<Self::ReadItem, LqError> {
        match id {
            TYPE_BOOL_TRUE => Result::Ok(true),
            TYPE_BOOL_FALSE => Result::Ok(false),
            _ => LqError::err_static("Type is not a boolean"),
        }
    }

    fn write<'b, Writer: BinaryWriter<'b> + 'b>(
        writer: Writer,
        item: &Self::WriteItem,
    ) -> Result<(), LqError> {
        match item {
            true => {
                writer.begin(TYPE_BOOL_TRUE)?;
                Result::Ok(())
            }
            false => {
                writer.begin(TYPE_BOOL_FALSE)?;
                Result::Ok(())
            }
        }
    }
}
