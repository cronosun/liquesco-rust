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
    Enum(EnumValue<'a>),
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
    Borrowed(&'a [Value<'a>]),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct EnumValue<'a> {
    ordinal: usize,
    value: Option<ValueRef<'a>>,
}

impl<'a> EnumValue<'a> {
    pub fn new_no_value(ordinal: usize) -> EnumValue<'static> {
        EnumValue {
            ordinal,
            value: Option::None,
        }
    }

    pub fn new_value<'b, T: Into<ValueRef<'b>>>(ordinal: usize, value: T) -> EnumValue<'b> {
        EnumValue {
            ordinal,
            value: Option::Some(value.into()),
        }
    }

    pub fn ordinal(&self) -> usize {
        self.ordinal
    }

    pub fn value(&self) -> &Option<ValueRef<'a>> {
        &self.value
    }
}

impl<'a> From<Value<'a>> for ValueRef<'a> {
    fn from(value: Value<'a>) -> Self {
        ValueRef::Boxed(Box::new(value))
    }
}

impl<'a> From<&'a Value<'a>> for ValueRef<'a> {
    fn from(value: &'a Value<'a>) -> Self {
        ValueRef::Borrowed(value)
    }
}

impl<'a> Deref for ValueList<'a> {
    type Target = [Value<'a>];

    fn deref(&self) -> &Self::Target {
        match self {
            ValueList::Borrowed(value) => value,
            ValueList::Owned(value) => value,
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
                    Value::Enum(EnumValue {
                        ordinal: enum_data.ordinal(),
                        value: Option::Some(ValueRef::Boxed(value)),
                    })
                } else {
                    Value::Enum(EnumValue {
                        ordinal: enum_data.ordinal(),
                        value: Option::None,
                    })
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
                Option::Some(present) => {
                    Presence::serialize(writer, &Presence::Present)?;
                    Value::serialize(writer, present)
                }
                Option::None => Presence::serialize(writer, &Presence::Absent),
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
            Value::Enum(value) => {
                let enum_data = if value.value.is_some() {
                    EnumData::new_with_value(value.ordinal)
                } else {
                    EnumData::new(value.ordinal)
                };
                EnumData::serialize(writer, &enum_data)?;
                if let Option::Some(some) = &value.value {
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
