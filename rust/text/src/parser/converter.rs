use crate::parser::value::Seq;
use crate::parser::value::TextValue;
use crate::parser::value::Text;
use crate::parser::value::Value;
use crate::parser::core::ParseError;
use liquesco_core::schema::identifier::Format;
use liquesco_core::schema::identifier::Identifier;
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
            IdentifierType::EnumIdentifier =>  identifier.to_string(Format::SnakeCase),
        }
    }

    fn string_to_identifier(value : &str, _ : IdentifierType) -> Result<Identifier, ParseError> {
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
            Value::F64(value) => {
                let s_value = *value;
                if s_value.is_sign_positive() && s_value.trunc() == s_value {
                    Option::Some(s_value as u64)
                } else {
                    Option::None
                }
            }
            _ => Option::None,
        }
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
            format!("Expecting to have a map with text keys (or a sequence with 0-n entries where each entry in \
            turn is a sequence with 2 elements where the first of those 2 elements is a text), got {:?}", value)
        })
    }

    fn require_u64(value: &Value) -> Result<u64, ParseError> {
        require(Self::to_u64(value), || {
            format!("Expecting an unsiged integer, got {:?}", value)
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
}

pub enum IdentifierType {
    StructField,
    EnumIdentifier,
}

fn require<T, Msg: FnOnce() -> String>(value: Option<T>, msg: Msg) -> Result<T, ParseError> {
    if let Some(value) = value {
        Result::Ok(value)
    } else {
        Result::Err(ParseError::new(msg()))
    }
}
