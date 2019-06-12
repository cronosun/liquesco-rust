use serde::de;
use serde::ser;
use std::fmt::Display;
use std::num::TryFromIntError;

use liquesco_common::error::{Category, LqError};

/// Errors resulting from serde problems.
pub(crate) const CATEGORY: Category = Category::new("liquesco_serde");

#[derive(Debug)]
pub(crate) struct SLqError(LqError);

impl Display for SLqError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
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
        Self(value.with_category(CATEGORY))
    }
}
