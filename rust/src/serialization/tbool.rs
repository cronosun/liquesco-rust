use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::LqError;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::Serializer;
use crate::serialization::type_ids::TYPE_BOOL_FALSE;
use crate::serialization::type_ids::TYPE_BOOL_TRUE;

impl<'a> DeSerializer<'a> for bool {
    type Item = Self;

    fn de_serialize<Reader: BinaryReader<'a>>(reader: &mut Reader) -> Result<Self::Item, LqError> {
        let id = reader.read_header_const(0)?;
        match id {
            TYPE_BOOL_TRUE => Result::Ok(true),
            TYPE_BOOL_FALSE => Result::Ok(false),
            _ => LqError::err_static("Type is not a boolean"),
        }
    }
}

impl Serializer for bool {
    type Item = Self;

    fn serialize<T: BinaryWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        match item {
            true => {
                writer.write_header_u8(TYPE_BOOL_TRUE, 0)?;
                Result::Ok(())
            }
            false => {
                writer.write_header_u8(TYPE_BOOL_FALSE, 0)?;                
                Result::Ok(())
            }
        }
    }
}
