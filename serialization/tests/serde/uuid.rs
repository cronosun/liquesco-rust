use crate::serde::assert_serde;
use crate::serde::serialize_to_same;
use liquesco_serialization::types::uuid::Uuid;
use serde_bytes;

#[test]
fn uuid_serde() {
    assert_serde(Uuid::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]));
}

/// By default serde serializes binary as vector of u8; we want compact serialization for uuid.
#[test]
fn uuid_encodes_compact() {
    let uuid = Uuid::from([47, 79, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    let uuid_as_bytes = serde_bytes::ByteBuf::from(uuid.as_slice().to_vec());
    serialize_to_same(uuid, uuid_as_bytes);
}
