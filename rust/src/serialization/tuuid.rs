use crate::serialization::binary::binary_read;
use crate::serialization::binary::binary_write;
use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::DeSerializer;
use crate::common::error::LqError;
use crate::serialization::core::Serializer;
use crate::serialization::type_ids::TYPE_UUID;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Uuid([u8; 16]);

impl<'a> DeSerializer<'a> for Uuid {
    type Item = Self;

    fn de_serialize<R: BinaryReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let (id, read_result) = binary_read(reader)?;
        if id != TYPE_UUID {
            return LqError::err_static("Type is not a UUID");
        }
        let mut uuid_bytes: [u8; 16] = [0; 16];
        let src_len = read_result.len();
        if src_len != 16 {
            return LqError::err_new(format!(
                "Invalid length of UUID (need to be 16 bytes; have {:?} bytes)",
                src_len
            ));
        }
        uuid_bytes.clone_from_slice(read_result);
        Result::Ok(Uuid(uuid_bytes))
    }
}

impl Serializer for Uuid {
    type Item = Self;

    fn serialize<W: BinaryWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        binary_write(&item.0, writer, TYPE_UUID)
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
