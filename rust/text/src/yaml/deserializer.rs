use crate::parser::value::{Value, TextValue, Seq};
use std::fmt;

use serde::de::{self, Visitor};
use std::collections::HashMap;
use std::borrow::Cow;

impl<'de> de::Deserialize<'de> for TextValue<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {        
        deserializer.deserialize_any(ValueVisitor)
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = TextValue<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Any value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> where
        E: de::Error, {
        Ok(Value::Bool(value).into())
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::I64(i64::from(value)).into())
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::I64(i64::from(value)).into())
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::I64(i64::from(value)).into())
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Text(Cow::Borrowed(v)).into())
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut result : Vec<Self::Value> = Vec::new();
        while let Some(entry) = seq.next_element::<Self::Value>()? {
            result.push(entry);
        }
        Ok(normalize_seq(Value::Seq(result).into()))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where
        A: de::MapAccess<'de>, {

        let mut result : Vec<Self::Value> = Vec::new();

        while let Some((key, value)) = map.next_entry()? {
            let entry = Value::Seq(vec![key, value]).into();
            result.push(entry);
        }
        Ok(normalize_seq(Value::Seq(result).into()))
    }
}

fn normalize_seq(text_value : TextValue) -> TextValue {
    if let Value::Seq(seq) = &text_value.value {
        if seq.len() == 1 {
            let entry = &seq[0].value;
            if let Value::Seq(entry_value) = entry {
                if entry_value.len() == 2 {
                    if let Value::Text(key) = &entry_value[0].value {
                        if key.starts_with("$") {
                            let mut key_string = key.to_string();
                            key_string.remove(0);
                            if key_string.starts_with("$") {
                                // escaped
                                key_string.remove(0);
                                return TextValue {
                                    name: Option::None,
                                    value: Value::Seq(vec![Value::Text(Cow::Owned(key_string)).into(), entry_value[1].clone()]).into(),
                                    position: text_value.position
                                }
                            } else {
                                // yes, that's it
                                return TextValue {
                                    name: Option::Some(Cow::Owned(key_string)),
                                    value: entry_value[1].clone().value,
                                    position: text_value.position
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    text_value
}

