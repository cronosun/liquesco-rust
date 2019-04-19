
use crate::serialization::core::LqError;

#[inline]
pub(crate) fn safe_read_u8(slice: &[u8], offset: usize) -> Result<u8, LqError> {
    if slice.len() <= offset {
        return LqError::err_static("Error reading from data: End of data (unable to read byte)");
    }
    Result::Ok(slice[offset])
}

#[inline]
pub(crate) fn safe_slice(
    slice: &[u8],
    start_index: usize,
    end_index: usize,
) -> Result<&[u8], LqError> {
    let slice_len = slice.len();
    if end_index > slice_len {
        return LqError::err_new(format!(
            "Error reading from data: End of data reached. Given slice has {:?} bytes, 
        start_index = {:?}, end_index = {:?}",
            slice_len, start_index, end_index
        ));
    }
    Result::Ok(&slice[start_index..end_index])
}

#[inline]
pub(crate) fn safe_slice_len(
    slice: &[u8],
    start_index: usize,
    len: usize,
) -> Result<&[u8], LqError> {
    let end_index = start_index + len;
    safe_slice(slice, start_index, end_index)
}

#[inline]
pub(crate) fn write_result<Ok>(result: Result<Ok, std::io::Error>) -> Result<Ok, LqError> {
    result.map_err(|_| LqError::new("Unable to write data"))
}