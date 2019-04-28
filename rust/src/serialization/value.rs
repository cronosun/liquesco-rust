use crate::serialization::tenum::EnumData;
use std::borrow::Cow;

pub enum Value<'a> {
    Bool(bool),
    Utf8(Cow<'a, str>),
    Binary(Cow<'a, [u8]>),
    Option(Option<&'a Value<'a>>),
    List(Cow<'a, [&'a Value<'a>]>),
    Enum((EnumData, Option<&'a Value<'a>>)),
    UInt(u64),
    SInt(i64),
}

impl From<bool> for Value<'static> {
    fn from(value : bool) -> Self {
        Value::Bool(value)
    }
}

impl From<String> for Value<'static> {
    fn from(value : String) -> Self {
        Value::Utf8(Cow::Owned(value))
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value : &'a str) -> Self {
        Value::Utf8(Cow::Borrowed(value))
    }
}

impl<'a> From<&'a [u8]> for Value<'a> {
    fn from(value : &'a [u8]) -> Self {
        Value::Binary(Cow::Borrowed(value))
    }
}

