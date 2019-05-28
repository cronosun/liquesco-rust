use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::num::TryFromIntError;

pub const DEFAULT_CATEGORY: Category = Category("default");
pub const DEFAULT_CODE: ErrCode = ErrCode(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Category(&'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrCode(usize);

#[derive(Debug)]
pub struct LqError {
    pub msg: Cow<'static, str>,
    pub category: Category,
    pub code: ErrCode,
    pub data: Option<HashMap<TypeId, Box<dyn ErrData>>>,
}

pub trait ErrData: Any + Send + Sync + Debug {}

impl Error for LqError {}

impl Display for LqError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LqError({:?})", self)
    }
}

impl LqError {
    // TODO: Can be removed?
    pub fn err_static<Ok>(string: &'static str) -> Result<Ok, LqError> {
        Result::Err(LqError {
            msg: string.into(),
            category: DEFAULT_CATEGORY,
            code: DEFAULT_CODE,
            data: None,
        })
    }

    pub fn new<T: Into<Cow<'static, str>>>(msg: T) -> Self {
        LqError {
            msg: msg.into(),
            category: DEFAULT_CATEGORY,
            code: DEFAULT_CODE,
            data: None,
        }
    }

    pub fn err_new<Ok, T: Into<Cow<'static, str>>>(msg: T) -> Result<Ok, Self> {
        Result::Err(Self::new(msg))
    }

    pub fn with_msg<T: Into<Cow<'static, str>>>(mut self, msg: T) -> LqError {
        self.msg = msg.into();
        self
    }

    pub fn msg(&self) -> &str {
        &self.msg
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
