use liquesco_common::error::LqError;
use crate::common_binary::binary_read;
use crate::common_binary::binary_write;
use crate::core::DeSerializer;
use crate::core::LqReader;
use crate::core::LqWriter;
use crate::core::Serializer;
use crate::major_types::TYPE_BINARY;

pub struct Binary;

impl<'a> DeSerializer<'a> for Binary {
    type Item = &'a [u8];

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let (id, read_result) = binary_read(reader)?;
        if id != TYPE_BINARY {
            return LqError::err_static("Type is not binary data");
        }
        Result::Ok(read_result)
    }
}

impl Serializer for Binary {
    type Item = [u8];

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        binary_write(item, writer, TYPE_BINARY)
    }
}
