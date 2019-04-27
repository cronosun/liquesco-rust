use crate::serialization::type_ids::TYPE_BINARY;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::Serializer;
use crate::serialization::core::BinaryReader;
use crate::serialization::binary::binary_write;
use crate::serialization::core::BinaryWriter;
use crate::serialization::binary::binary_read;
use crate::serialization::core::LqError;

pub struct TBinary;

impl<'a> DeSerializer<'a> for TBinary {
    type Item = &'a [u8];

    fn de_serialize<Reader : BinaryReader<'a>>(reader: &mut Reader) -> Result<Self::Item, LqError> {
        let (id, read_result) = binary_read(reader)?;
        if id!=TYPE_BINARY {
            return LqError::err_static("Type is not binary data");
        }
       Result::Ok(read_result)
    }    
}

impl Serializer for TBinary {

    type Item = [u8];

    fn serialize<'b, T: BinaryWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        binary_write(item, writer, TYPE_BINARY)
    }
}