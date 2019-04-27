use crate::serialization::core::Serializer;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::BinaryReader;
use crate::serialization::binary::binary_write;
use crate::serialization::core::BinaryWriter;
use crate::serialization::type_ids::TYPE_UTF8;
use crate::serialization::binary::binary_read;
use crate::serialization::core::LqError;
use std::str::from_utf8;

pub struct TUtf8;

impl<'a> DeSerializer<'a> for TUtf8 {
    type Item = &'a str;

    fn de_serialize<Reader : BinaryReader<'a>>(reader: &mut Reader) -> Result<Self::Item, LqError> {
        let (id, read_result) = binary_read(reader)?;
        if id!=TYPE_UTF8 {
            return LqError::err_new(format!("Type is not utf8 data, id is {:?}",
            id));
        }
        let maybe_str = from_utf8(read_result);
        match maybe_str {
            Result::Ok(value) => Result::Ok(value),
            Result::Err(_) => LqError::err_static("Invalid utf8 data found"),
        }
    }
}

impl Serializer for TUtf8 {
    type Item = str;

    fn serialize<'b, T: BinaryWriter>(
        writer: &mut T,
        item: &Self::Item,
    ) -> Result<(), LqError> {
        let as_utf8 = item.as_bytes();
        binary_write(as_utf8, writer, TYPE_UTF8)
    }
}
