use crate::serialization::binary::binary_write;
use crate::serialization::core::BinaryWriter;
use crate::serialization::types::BLOCK_ID_UTF8;
use crate::serialization::binary::binary_read;
use crate::serialization::core::LqError;
use crate::serialization::core::TypeId;
use crate::serialization::core::ReadResult;
use crate::serialization::core::Type;

pub struct TBinary;

impl<'a> Type<'a> for TBinary {
    type ReadItem = &'a [u8];
    type WriteItem = [u8];

    fn read(id: TypeId, data: &'a [u8]) -> Result<ReadResult<Self::ReadItem>, LqError> {
        let (block, read_result) = binary_read(id, data)?;
        if block!=BLOCK_ID_UTF8 {
            return LqError::err_static("Type is not binary data");
        }
       Result::Ok(read_result)
    }

    fn write<'b, Writer: BinaryWriter<'b> + 'b>(
        writer: Writer,
        item: &Self::WriteItem,
    ) -> Result<(), LqError> {
        binary_write(item, writer, BLOCK_ID_UTF8)
    }
}