use crate::parser::core::ParseError;
use crate::parser::value::Seq;
use crate::parser::value::Text;
use crate::parser::value::TextValue;
use crate::parser::value::Value;
use liquesco_schema::identifier::Format;
use liquesco_schema::identifier::Identifier;
use std::collections::HashMap;
use std::convert::TryFrom;

/// Naming:
/// - require: Returns an error when conversion is not possible.
/// - to: converts to some optional.
pub trait Converter {
    /// Converts an identifier to a string suited for the text format.
    fn identifier_to_string(identifier: &Identifier, what_for: IdentifierType) -> String {
        match what_for {
            IdentifierType::StructField => identifier.to_string(Format::SnakeCase),
            IdentifierType::EnumIdentifier => identifier.to_string(Format::SnakeCase),
        }
    }

    fn string_to_identifier(value: &str, _: IdentifierType) -> Result<Identifier, ParseError> {
        Ok(Identifier::try_from(value)?)
    }

    fn to_text<'a>(value: &'a Value<'a>) -> Option<&'a Text<'a>> {
        match value {
            Value::Text(value) => Option::Some(value),
            _ => Option::None,
        }
    }

    fn require_text<'a>(value: &'a Value<'a>) -> Result<&'a Text<'a>, ParseError> {
        require(Self::to_text(value), || {
            format!("Expecting to have a string; got {:?}", value)
        })
    }

    fn to_u64(value: &Value) -> Option<u64> {
        match value {
            Value::U64(value) => Option::Some(*value),
            Value::I64(value) => {
                let s_value = *value;
                if s_value > 0 {
                    Option::Some(s_value as u64)
                } else {
                    Option::None
                }
            }
            // TODO: Maybe also allow "MAX_8", "MAX_16", "MIN_8"...
            Value::Text(text) => text.parse::<u64>().ok(),
            _ => Option::None,
        }
    }

    fn require_u64(value: &Value) -> Result<u64, ParseError> {
        require(Self::to_u64(value), || {
            format!("Expecting an unsiged integer, got {:?}", value)
        })
    }

    fn to_bool(value: &Value) -> Option<bool> {
        match value {
            Value::Bool(value) => Option::Some(*value),
            Value::Text(value) => {
                let value_str: &str = &value;
                match value_str {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => None,
                }
            }
            _ => Option::None,
        }
    }

    fn require_bool(value: &Value) -> Result<bool, ParseError> {
        require(Self::to_bool(value), || {
            format!("Expecting a boolean, got {:?}", value)
        })
    }

    fn to_i64(value: &Value) -> Option<i64> {
        match value {
            Value::I64(value) => Option::Some(*value),
            Value::U64(value) => {
                if value < &(std::i64::MAX as u64) {
                    Some(*value as i64)
                } else {
                    Option::None
                }
            }
            Value::Text(text) => text.parse::<i64>().ok(),
            _ => Option::None,
        }
    }

    fn require_i64(value: &Value) -> Result<i64, ParseError> {
        require(Self::to_i64(value), || {
            format!("Expecting a signed integer, got {:?}", value)
        })
    }

    fn to_f64(value: &Value) -> Option<f64> {
        match value {
            Value::F64(value) => Option::Some(*value),
            Value::Text(value) => value.parse::<f64>().ok(),
            _ => None,
        }
    }

    fn require_f64(value: &Value) -> Result<f64, ParseError> {
        require(Self::to_f64(value), || {
            format!(
                "Expecting a float 64 (if this is an integer, try adding .0; e.g. 12 \
                 -> 12.0), got {:?}",
                value
            )
        })
    }

    fn to_f32(value: &Value) -> Option<f32> {
        match value {
            Value::F64(value) => {
                // accept when no precision is lost
                let f32_value = *value as f32;
                let f64_value = f32_value as f64;
                if value == &f64_value {
                    Some(f32_value)
                } else {
                    None
                }
            }
            Value::Text(value) => value.parse::<f32>().ok(),
            _ => None,
        }
    }

    fn require_f32(value: &Value) -> Result<f32, ParseError> {
        require(Self::to_f32(value), || {
            format!(
                "Expecting a float 32 (if this is an integer, try adding .0; e.g. 12 \
                 -> 12.0; of it looks like a float, make sure it can be represented as 32 bit \
                 float value without loosing precision), got {:?}",
                value
            )
        })
    }

    fn to_string_map<'a>(value: &'a Value<'a>) -> Option<HashMap<&'a str, &'a TextValue<'a>>> {
        if let Value::Seq(seq) = value {
            let mut result: HashMap<&'a str, &'a TextValue<'a>> = HashMap::with_capacity(seq.len());
            for entry in seq {
                if let Value::Seq(key_value) = &entry.value {
                    if key_value.len() == 2 {
                        if let Some(key) = Self::to_text(&key_value[0].value) {
                            result.insert(key, &key_value[1]);
                        } else {
                            return Option::None;
                        }
                    } else {
                        return Option::None;
                    }
                } else {
                    return Option::None;
                }
            }
            Option::Some(result)
        } else {
            Option::None
        }
    }

    fn require_string_map<'a>(
        value: &'a Value<'a>,
    ) -> Result<HashMap<&'a str, &'a TextValue<'a>>, ParseError> {
        require(Self::to_string_map(value), || {
            format!(
                "Expecting to have a map with text keys (or a sequence with 0-n \
                 entries where each entry in turn is a sequence with 2 elements where the first \
                 of those 2 elements is a text), got {:?}",
                value
            )
        })
    }

    fn require_no_name(value: &TextValue) -> Result<(), ParseError> {
        if let Some(name) = &value.name {
            Result::Err(ParseError::new(format!(
                "The given value has a name (name \
                 is {:?}). This name is unused and must be removed. Value is {:?}.",
                name, value
            )))
        } else {
            Result::Ok(())
        }
    }

    fn to_seq<'a>(value: &'a Value<'a>) -> Option<&'a Seq<'a>> {
        if let Value::Seq(value) = value {
            Some(value)
        } else {
            None
        }
    }

    fn require_seq<'a>(value: &'a Value<'a>) -> Result<&'a Seq<'a>, ParseError> {
        require(Self::to_seq(value), || {
            format!(
                "Expecting to have a sequence (aka. list or vector). got {:?}",
                value
            )
        })
    }

    fn master_anchor() -> &'static str {
        "MAIN*"
    }

    fn validate_reference(value: &str) -> Result<(), ParseError> {
        if value == Self::master_anchor() {
            // of course the master anchor is OK
            Result::Ok(())
        } else {
            if !value.ends_with("*") {
                return Err(ParseError::new(format!(
                    "References must end with `*`; \
                     the reference you supplied does not: `{:?}`",
                    value
                )));
            }
            let len = value.len();
            let (identifier, _) = value.split_at(len - 1);
            if Identifier::try_from(identifier).is_err() {
                return Err(ParseError::new(format!(
                    "References must be valid \
                     identifiers (see doc; essentially only latin lower cases and underscores). \
                     You supplied: `{:?}`",
                    identifier
                )));
            }
            Result::Ok(())
        }
    }
}

pub enum IdentifierType {
    StructField,
    EnumIdentifier, // TODO: Vielleicht das in klammern?
}

fn require<T, Msg: FnOnce() -> String>(value: Option<T>, msg: Msg) -> Result<T, ParseError> {
    if let Some(value) = value {
        Result::Ok(value)
    } else {
        Result::Err(ParseError::new(msg()))
    }
}
