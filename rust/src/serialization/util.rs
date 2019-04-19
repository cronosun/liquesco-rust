
use crate::serialization::core::LqError;

#[inline]
pub(crate) fn io_result<Ok>(result: Result<Ok, std::io::Error>) -> Result<Ok, LqError> {
    result.map_err(|_| LqError::new("Unable to write data"))
}