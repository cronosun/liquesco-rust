use crate::value::TextValue;
use crate::value::Value;
use liquesco_common::error::LqError;
use std::borrow::Cow;
use yaml_rust::Yaml;

pub(crate) fn deserialize(yaml: Yaml) -> Result<TextValue<'static>, LqError> {
    deserialize_single(yaml)
}

fn deserialize_single(yaml: Yaml) -> Result<TextValue<'static>, LqError> {
    Result::Ok(
        match yaml {
            Yaml::Null => Value::Nothing,
            Yaml::String(string) => Value::Text(Cow::Owned(string)),
            Yaml::Boolean(value) => Value::Bool(value),
            Yaml::Array(array) => {
                let mut vec = Vec::new();
                for item in array {
                    vec.push(deserialize_single(item)?)
                }
                Value::Seq(vec).into()
            }
            Yaml::Hash(hash) => {
                let mut vec = Vec::new();
                for (key, value) in hash {
                    let entry_seq = vec![deserialize_single(key)?, deserialize_single(value)?];
                    vec.push(Value::Seq(entry_seq).into());
                }
                Value::Seq(vec).into()
            }
            Yaml::Integer(integer) => Value::I64(integer).into(),
            Yaml::Real(real) => Value::Text(Cow::Owned(real)),
            _ => {
                return Result::Err(LqError::new(format!(
                    "Unable to parse yaml, \
                     unhandled element: {:?}",
                    yaml
                )))
            }
        }
        .into(),
    )
}
