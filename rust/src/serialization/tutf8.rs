use crate::serialization::core::TypeWriter;
use crate::serialization::core::TypeReader;
use crate::serialization::core::BinaryReader;
use crate::serialization::binary::binary_write;
use crate::serialization::core::BinaryWriter;
use crate::serialization::type_ids::BLOCK_ID_UTF8;
use crate::serialization::binary::binary_read;
use crate::serialization::core::LqError;
use crate::serialization::core::TypeId;
use std::str::from_utf8;

pub struct TUtf8;

impl<'a> TypeReader<'a> for TUtf8 {
    type Item = &'a str;

    fn read<Reader : BinaryReader<'a>>(id: TypeId, reader: &mut Reader) -> Result<Self::Item, LqError> {
        let (block, read_result) = binary_read(id, reader)?;
        if block!=BLOCK_ID_UTF8 {
            return LqError::err_new(format!("Type is not utf8 data, block is {:?}",
            block));
        }
        let maybe_str = from_utf8(read_result);
        match maybe_str {
            Result::Ok(value) => Result::Ok(value),
            Result::Err(_) => LqError::err_static("Invalid utf8 data found"),
        }
    }
}

impl TypeWriter for TUtf8 {
    type Item = str;

    fn write<'b, Writer: BinaryWriter<'b> + 'b>(
        writer: Writer,
        item: &Self::Item,
    ) -> Result<(), LqError> {
        let as_utf8 = item.as_bytes();
        binary_write(as_utf8, writer, BLOCK_ID_UTF8)
    }
}
