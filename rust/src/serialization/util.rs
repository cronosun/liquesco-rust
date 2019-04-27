use crate::serialization::core::LqError;
use std::num::TryFromIntError;

#[inline]
pub(crate) fn io_result<Ok>(result: Result<Ok, std::io::Error>) -> Result<Ok, LqError> {
    result.map_err(|_| LqError::new("Unable to write data"))
}

#[inline]
pub(crate) fn try_from_int_result<Ok>(result: Result<Ok, TryFromIntError>) -> Result<Ok, LqError> {
    result.map_err(|err| {
        LqError::new(format!(
            "The given integers could not be converted (casted); this 
    can happen on platforms with small usize. In general this library only works with things as 
    big as this platform supports; error: {:?}",
            err
        ))
    })
}
