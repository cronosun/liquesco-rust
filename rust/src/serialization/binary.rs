use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::common::error::LqError;
use crate::serialization::core::TypeId;
use crate::serialization::util::io_result;

#[inline]
pub(crate) fn binary_write<T: BinaryWriter>(
    data: &[u8],
    writer: &mut T,
    type_id: TypeId
) -> Result<(), LqError> {
    let bin_len = data.len();
    writer.write_header_usize(type_id, bin_len)?;
    io_result(writer.write(data))?;
    Result::Ok(())
}

#[inline]
pub(crate) fn binary_read<'a, Reader: BinaryReader<'a>>(
    reader: &mut Reader,
) -> Result<(TypeId, &'a [u8]), LqError> {
    let (type_id, len) = reader.read_header_usize()?;
    let read_result = reader.read_slice(len)?;
    Result::Ok((type_id, read_result))
}
