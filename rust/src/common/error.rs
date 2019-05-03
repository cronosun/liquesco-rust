
use std::borrow::Cow;
use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]
pub struct LqError {
    pub msg: Cow<'static, str>,
}

impl Error for LqError {}

impl Display for LqError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LqError({:?})", self.msg)
    }
}

impl LqError {
    pub fn err_static<Ok>(string: &'static str) -> Result<Ok, LqError> {
        Result::Err(LqError { msg: string.into() })
    }

    pub fn new<T: Into<Cow<'static, str>>>(msg: T) -> Self {
        LqError { msg: msg.into() }
    }

    pub fn err_new<Ok, T: Into<Cow<'static, str>>>(msg: T) -> Result<Ok, Self> {
        Result::Err(Self::new(msg))
    }

    pub fn with_msg<T: Into<Cow<'static, str>>>(mut self, msg: T) -> LqError {
        self.msg = msg.into();
        self
    }
}