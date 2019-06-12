use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::num::{ParseIntError, TryFromIntError};

pub const DEFAULT_CATEGORY: Category = Category("default");
pub const DEFAULT_CODE: ErrCode = ErrCode(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Category(&'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrCode(usize);

/// A liquesco error. Has a message, a category, an error code and optionally some data.
#[derive(Debug)]
pub struct LqError {
    msg: Cow<'static, str>,
    category: Category,
    code: ErrCode,
    data: Option<HashMap<TypeId, Box<dyn ErrData>>>,
}

pub trait ErrData: Any + Send + Sync + Debug {}

impl Error for LqError {}

impl Display for LqError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LqError({:?})", self)
    }
}

impl Category {
    pub const fn new(string: &'static str) -> Self {
        Self(string)
    }
}

impl LqError {
    /// Creates a new error with a message.
    pub fn new<T: Into<Cow<'static, str>>>(msg: T) -> Self {
        LqError {
            msg: msg.into(),
            category: DEFAULT_CATEGORY,
            code: DEFAULT_CODE,
            data: None,
        }
    }

    /// Creates a new `Result::Err` with a message.
    pub fn err_new<Ok, T: Into<Cow<'static, str>>>(msg: T) -> Result<Ok, Self> {
        Result::Err(Self::new(msg))
    }

    /// With a different message.
    pub fn with_msg<T: Into<Cow<'static, str>>>(mut self, msg: T) -> LqError {
        self.msg = msg.into();
        self
    }

    /// The message.
    pub fn msg(&self) -> &str {
        &self.msg
    }

    /// With a different category.
    pub fn with_category(mut self, category: Category) -> Self {
        self.category = category;
        self
    }

    /// The category of this error.
    pub fn category(&self) -> &Category {
        &self.category
    }
}

impl From<TryFromIntError> for LqError {
    fn from(value: TryFromIntError) -> Self {
        LqError::new(format!(
            "The given integers could not be converted (casted); this \
             can either happen on platforms with small usize (in general this library only works \
             with things as big as this platform supports) - or there's a serialization \
             problem; error: {:?}",
            value
        ))
    }
}

impl From<std::io::Error> for LqError {
    fn from(value: std::io::Error) -> Self {
        LqError::new(format!("Got an I/O error: {:?}", value))
    }
}

impl From<ParseIntError> for LqError {
    fn from(value: ParseIntError) -> Self {
        LqError::new(format!(
            "Unable to parse given integer (converting from string to integer); error: {:?}",
            value
        ))
    }
}
