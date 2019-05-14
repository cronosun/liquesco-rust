use yaml_rust::{Yaml};
use crate::parser::value::TextValue;
use crate::parser::value::Value;
use crate::parser::core::ParseError;
use std::borrow::Cow;

pub(crate) fn deserialize(yaml : Yaml) -> Result<TextValue<'static>, ParseError> {
    deserialize_single(yaml)
}

fn deserialize_single(yaml : Yaml) -> Result<TextValue<'static>, ParseError> {
    Result::Ok(match yaml {
        Yaml::Null => Value::Maybe(Option::None),
        Yaml::String(string) => Value::Text(Cow::Owned(string)),
        Yaml::Boolean(value) => Value::Bool(value),
        Yaml::Array(array) => {
            let mut vec = Vec::new();
            for item in array {
                vec.push(deserialize_single(item)?)
            }
            Value::Seq(vec).into()
        }
        _ => {
            return Result::Err(ParseError::new("Unable to parse yaml"))
        }
    }.into())
}