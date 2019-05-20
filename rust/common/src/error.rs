use std::borrow::Cow;
use std::error::Error;
use std::fmt::Display;
use std::num::TryFromIntError;

#[derive(Debug, Clone, PartialEq, Eq)]
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
