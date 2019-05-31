use crate::serde::assert_serde;
use crate::serde::serialize_to_same;
use liquesco_serialization::uuid::Uuid;
use serde_bytes;

#[test]
fn uuid_serde() {
    assert_serde(Uuid::new_v4());
}

/// By default serde serializes binary as vector of u8; we want compact serialization for uuid.
#[test]
fn uuid_encodes_compact() {
    let uuid = Uuid::new_v4();
    let uuid_as_bytes = serde_bytes::ByteBuf::from(uuid.as_slice().to_vec());
    serialize_to_same(uuid, uuid_as_bytes);
}

