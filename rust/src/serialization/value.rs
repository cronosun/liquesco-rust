use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::LqError;
use crate::serialization::core::Serializer;
use crate::serialization::tbinary::TBinary;
use crate::serialization::tenum::EnumData;
use crate::serialization::tlist::ListData;
use crate::serialization::toption::Presence;
use crate::serialization::tsint::TSInt;
use crate::serialization::tuint::TUInt;
use crate::serialization::tutf8::TUtf8;
use crate::serialization::type_ids::TYPE_BINARY;
use crate::serialization::type_ids::TYPE_BOOL_FALSE;
use crate::serialization::type_ids::TYPE_BOOL_TRUE;
use crate::serialization::type_ids::TYPE_ENUM_0;
use crate::serialization::type_ids::TYPE_ENUM_1;
use crate::serialization::type_ids::TYPE_ENUM_2;
use crate::serialization::type_ids::TYPE_ENUM_N;
use crate::serialization::type_ids::TYPE_LIST;
use crate::serialization::type_ids::TYPE_OPTION;
use crate::serialization::type_ids::TYPE_SINT;
use crate::serialization::type_ids::TYPE_UINT;
use crate::serialization::type_ids::TYPE_UTF8;
use std::ops::Deref;

use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Value<'a> {
    Bool(bool),
    Utf8(Cow<'a, str>),
    Binary(Cow<'a, [u8]>),
    Option(Option<ValueRef<'a>>),
    List(ValueList<'a>), 
    Enum((usize, Option<ValueRef<'a>>)),
    UInt(u64),
    SInt(i64),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ValueRef<'a> {
    Borrowed(&'a Value<'a>),
    Boxed(Box<Value<'a>>),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ValueList<'a> {
    Owned(Vec<Value<'a>>),
    Borrowed(&'a [Value<'a>])
}

impl<'a> Deref for ValueList<'a> {
    type Target = [Value<'a>];

    fn deref(&self) -> &Self::Target {
        match self {
            ValueList::Borrowed(value) => value,
            ValueList::Owned(value) => value
        }
    }
}

impl<'a> Deref for ValueRef<'a> {
    type Target = Value<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            ValueRef::Borrowed(value) => *value,
            ValueRef::Boxed(value) => &value,
        }
    }
}

impl From<bool> for Value<'static> {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<String> for Value<'static> {
    fn from(value: String) -> Self {
        Value::Utf8(Cow::Owned(value))
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Self {
        Value::Utf8(Cow::Borrowed(value))
    }
}

impl<'a> From<&'a [u8]> for Value<'a> {
    fn from(value: &'a [u8]) -> Self {
        Value::Binary(Cow::Borrowed(value))
    }
}

impl<'a> DeSerializer<'a> for Value<'a> {
    type Item = Self;

    fn de_serialize<Reader: BinaryReader<'a>>(reader: &mut Reader) -> Result<Self::Item, LqError> {
        let type_id = reader.peek_header()?.type_id();
        let value = match type_id {
            TYPE_BOOL_FALSE | TYPE_BOOL_TRUE => Value::Bool(bool::de_serialize(reader)?),
            TYPE_OPTION => {
                let presence = Presence::de_serialize(reader)?;
                match presence {
                    Presence::Present => Value::Option(Option::Some(ValueRef::Boxed(Box::new(
                        Value::de_serialize(reader)?,
                    )))),
                    Presence::Absent => Value::Option(Option::None),
                }
            }
            TYPE_LIST => {
                let list_data = ListData::de_serialize(reader)?;
                let length = list_data.length();
                let mut vec = Vec::with_capacity(length);
                for _ in 0..length {
                    vec.push(Value::de_serialize(reader)?);
                }
                Value::List(ValueList::Owned(vec))
            }
            TYPE_BINARY => {
                let bin = TBinary::de_serialize(reader)?;
                Value::Binary(Cow::Borrowed(bin))
            }
            TYPE_UTF8 => {
                let string = TUtf8::de_serialize(reader)?;
                Value::Utf8(Cow::Borrowed(string))
            }
            TYPE_ENUM_0 | TYPE_ENUM_1 | TYPE_ENUM_2 | TYPE_ENUM_N => {
                let enum_data = EnumData::de_serialize(reader)?;
                if enum_data.has_value() {
                    let value = Box::new(Value::de_serialize(reader)?);
                    Value::Enum((enum_data.ordinal(), Option::Some(ValueRef::Boxed(value))))
                } else {
                    Value::Enum((enum_data.ordinal(), Option::None))
                }
            }
            TYPE_UINT => {
                let value = TUInt::de_serialize(reader)?;
                Value::UInt(value)
            }
            TYPE_SINT => {
                let value = TSInt::de_serialize(reader)?;
                Value::SInt(value)
            }
            _ => {
                return LqError::err_new(format!("Unknown type {:?}", type_id));
            }
        };
        Result::Ok(value)
    }
}

impl<'a> Serializer for Value<'a> {
    type Item = Self;

    fn serialize<T: BinaryWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        match item {
            Value::Bool(value) => bool::serialize(writer, value),
            Value::Option(value) => match value {
                Option::Some(present) => Value::serialize(writer, present),
                Option::None => Result::Ok(()),
            },
            Value::List(value) => {
                let len = value.len();
                let list_data = ListData::new(len);
                ListData::serialize(writer, &list_data)?;
                for item in value.deref() {
                    Value::serialize(writer, item)?;
                }
                Result::Ok(())
            }
            Value::Binary(value) => TBinary::serialize(writer, value),
            Value::Utf8(value) => TUtf8::serialize(writer, value),
            Value::Enum((ordinal, maybe_value)) => {
                let enum_data = if maybe_value.is_some() {
                    EnumData::new_with_value(*ordinal)
                } else {
                    EnumData::new(*ordinal)
                };
                EnumData::serialize(writer, &enum_data)?;
                if let Option::Some(some) = maybe_value {
                    Value::serialize(writer, some)
                } else {
                    Result::Ok(())
                }
            }
            Value::UInt(value) => TUInt::serialize(writer, value),
            Value::SInt(value) => TSInt::serialize(writer, value),
        }
    }
}
