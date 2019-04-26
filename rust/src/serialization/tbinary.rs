use crate::serialization::type_ids::BLOCK_ID_BINARY;
use crate::serialization::core::TypeReader;
use crate::serialization::core::TypeWriter;
use crate::serialization::core::BinaryReader;
use crate::serialization::binary::binary_write;
use crate::serialization::core::BinaryWriter;
use crate::serialization::binary::binary_read;
use crate::serialization::core::LqError;
use crate::serialization::core::TypeId;

pub struct TBinary;

impl<'a> TypeReader<'a> for TBinary {
    type Item = &'a [u8];

    fn read<Reader : BinaryReader<'a>>(id: TypeId, reader: &mut Reader) -> Result<Self::Item, LqError> {
        let (block, read_result) = binary_read(id, reader)?;
        if block!=BLOCK_ID_BINARY {
            return LqError::err_static("Type is not binary data");
        }
       Result::Ok(read_result)
    }    
}

impl TypeWriter for TBinary {

    type Item = [u8];

    fn write<'b, Writer: BinaryWriter<'b> + 'b>(
        writer: Writer,
        item: &Self::Item,
    ) -> Result<(), LqError> {
        binary_write(item, writer, BLOCK_ID_BINARY)
    }
}