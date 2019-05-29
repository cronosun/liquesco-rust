use crate::binary::Binary;
use crate::core::DeSerializer;
use crate::core::LqReader;
use crate::core::LqWriter;
use crate::core::Serializer;
use liquesco_common::error::LqError;
use serde::{Deserialize, Serialize};

/// 16 byte Universally Unique Identifier (UUID) according to RFC 4122. Does not specify which
/// variant is allowed. Does not validate: Any 16 byte binary is allowed.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Uuid([u8; 16]);

impl<'a> DeSerializer<'a> for Uuid {
    type Item = Self;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        // it's just a normal binary
        let binary = Binary::de_serialize(reader)?;
        let mut uuid_bytes: [u8; 16] = [0; 16];
        let src_len = binary.len();
        if src_len != 16 {
            return LqError::err_new(format!(
                "Invalid length of UUID (need to be 16 bytes; have {:?} bytes)",
                src_len
            ));
        }
        uuid_bytes.clone_from_slice(binary);
        Result::Ok(Uuid(uuid_bytes))
    }
}

impl Serializer for Uuid {
    type Item = Self;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        Binary::serialize(writer, &item.0)
    }
}

impl Uuid {
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn new_v4() -> Self {
        let new_v4 = uuid::Uuid::new_v4();
        Self(new_v4.as_bytes().clone())
    }
}

impl From<[u8; 16]> for Uuid {
    fn from(value: [u8; 16]) -> Self {
        Self(value)
    }
}

impl From<&[u8; 16]> for Uuid {
    fn from(value: &[u8; 16]) -> Self {
        Self(value.clone())
    }
}
