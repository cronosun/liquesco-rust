use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::LqError;
use crate::serialization::core::TypeId;
use crate::serialization::core::TypeReader;
use crate::serialization::core::TypeWriter;
use crate::serialization::types::TYPE_BOOL_FALSE;
use crate::serialization::types::TYPE_BOOL_TRUE;

pub struct TBool;

impl<'a> TypeReader<'a> for TBool {
    type Item = bool;

    fn read<Reader: BinaryReader>(id: TypeId, _: &'a mut Reader) -> Result<Self::Item, LqError> {
        match id {
            TYPE_BOOL_TRUE => Result::Ok(true),
            TYPE_BOOL_FALSE => Result::Ok(false),
            _ => LqError::err_static("Type is not a boolean"),
        }
    }
}

impl TypeWriter for TBool {
    type Item = bool;

    fn write<'b, Writer: BinaryWriter<'b> + 'b>(
        writer: Writer,
        item: &Self::Item,
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
