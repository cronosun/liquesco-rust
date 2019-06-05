use crate::core::LqReader;
use crate::core::LqWriter;
use crate::serde::deserializer::Deserializer;
use crate::serde::error::CATEGORY;
use crate::serde::serializer::Serializer;
use crate::slice_reader::SliceReader;
use crate::vec_writer::VecWriter;
use liquesco_common::error::{Category, LqError};
use serde::ser;
use crate::core::ToVecLqWriter;

mod deserializer;
mod error;
mod serializer;

/// Returns the serde error category. All errors related to serde problems should
/// have that category set.
pub const fn serde_error_category() -> Category {
    CATEGORY
}

/// Serializes given item. When an error occurs the state of the writer is undefined.
#[inline]
pub fn serialize<W: LqWriter, S: ser::Serialize>(writer: &mut W, value: S) -> Result<(), LqError> {
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer).map_err(|err| err.into())
}

/// Serializes given item and returns the result as `Vec<u8>`.
#[inline]
pub fn serialize_to_vec<S: ser::Serialize>(value: S) -> Result<Vec<u8>, LqError> {
    let mut vec_writer = VecWriter::default();
    serialize(&mut vec_writer, value)?;
    Ok(vec_writer.into_vec())
}

/// De-serializes `T` using given reader. When an error occurs the state of the reader is
/// undefined and should not longer be used.
pub fn de_serialize<'de, T: serde::Deserialize<'de>, R: LqReader<'de>>(
    reader: R,
) -> Result<T, LqError> {
    let mut de_serializer = Deserializer::new(reader);
    T::deserialize(&mut de_serializer).map_err(|err| err.into())
}

/// De-serializes `T` from given slice.
pub fn de_serialize_from_slice<'de, T: serde::Deserialize<'de>>(
    slice: &'de [u8],
) -> Result<T, LqError> {
    let reader: SliceReader = slice.into();
    de_serialize(reader)
}
