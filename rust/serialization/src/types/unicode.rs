use crate::types::common_binary::binary_read;
use crate::types::common_binary::binary_write;
use crate::core::DeSerializer;
use crate::core::LqReader;
use crate::core::LqWriter;
use crate::core::Serializer;
use crate::major_types::TYPE_UNICODE;
use liquesco_common::error::LqError;
use std::str::from_utf8;

/// Unicode (UTF-8) text.
pub struct Unicode;

impl<'a> DeSerializer<'a> for Unicode {
    type Item = &'a str;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let (id, read_result) = binary_read(reader)?;
        if id != TYPE_UNICODE {
            return LqError::err_new(format!("Type is not utf8 data, id is {:?}", id));
        }
        let maybe_str = from_utf8(read_result);
        match maybe_str {
            Result::Ok(value) => Result::Ok(value),
            Result::Err(_) => LqError::err_new("Invalid utf8 data found"),
        }
    }
}

impl Serializer for Unicode {
    type Item = str;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        let as_utf8 = item.as_bytes();
        binary_write(as_utf8, writer, TYPE_UNICODE)
    }
}

/// Use this to ready unicode text without check for UTF-8 validity. This is for example used
/// to read ASCII text (in the case of ASCII text the schema will perform validity check).
pub struct UncheckedUnicode;

impl<'a> DeSerializer<'a> for UncheckedUnicode {
    type Item = &'a [u8];

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let (id, read_result) = binary_read(reader)?;
        if id != TYPE_UNICODE {
            return LqError::err_new(format!("Type is not utf8 data, id is {:?}", id));
        }
        Result::Ok(read_result)
    }
}

impl Serializer for UncheckedUnicode {
    type Item = [u8];

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        binary_write(item, writer, TYPE_UNICODE)
    }
}
