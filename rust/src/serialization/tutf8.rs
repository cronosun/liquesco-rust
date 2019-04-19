use crate::serialization::binary::binary_write;
use crate::serialization::core::BinaryWriter;
use crate::serialization::types::BLOCK_ID_UTF8;
use crate::serialization::binary::binary_read;
use crate::serialization::core::LqError;
use crate::serialization::core::TypeId;
use crate::serialization::core::ReadResult;
use crate::serialization::core::Type;
use std::str::from_utf8;

pub struct TUtf8;

impl<'a> Type<'a> for TUtf8 {
    type ReadItem = &'a str;
    type WriteItem = str;

    fn read(id: TypeId, data: &'a [u8]) -> Result<ReadResult<Self::ReadItem>, LqError> {
        let (block, read_result) = binary_read(id, data)?;
        if block!=BLOCK_ID_UTF8 {
            return LqError::err_static("Type is not utf8 data");
        }
        let maybe_str = from_utf8(read_result.data);
        match maybe_str {
            Result::Ok(value) => ReadResult::new_ok(read_result.num_read, value),
            Result::Err(_) => LqError::err_static("Invalid utf8 data found"),
        }
    }

    fn write<'b, Writer: BinaryWriter<'b> + 'b>(
        writer: Writer,
        item: &Self::WriteItem,
    ) -> Result<(), LqError> {
        let as_utf8 = item.as_bytes();
        binary_write(as_utf8, writer, BLOCK_ID_UTF8)
    }
}
