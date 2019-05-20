use serde::de;
use serde::ser;
use std::fmt::Display;
use std::num::TryFromIntError;

use liquesco_common::error::LqError;

// TODO: Use LqError with additional information!
#[derive(Debug)]
pub struct SLqError(LqError);

impl Display for SLqError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SLqError({:?})", self.0.msg)
    }
}

impl ser::Error for SLqError {
    fn custom<T: Display>(msg: T) -> Self {
        LqError::new(msg.to_string()).into()
    }
}

impl de::Error for SLqError {
    fn custom<T: Display>(msg: T) -> Self {
        LqError::new(msg.to_string()).into()
    }
}

impl std::error::Error for SLqError {}

impl From<TryFromIntError> for SLqError {
    fn from(value: TryFromIntError) -> Self {
        let lq_error: LqError = value.into();
        lq_error.into()
    }
}

impl From<SLqError> for LqError {
    fn from(value: SLqError) -> Self {
        value.0
    }
}

impl From<LqError> for SLqError {
    fn from(value: LqError) -> Self {
        Self(value)
    }
}
