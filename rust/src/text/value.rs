use crate::schema::identifier::Format;
use crate::schema::identifier::Identifier;
use crate::text::core::ParseError;
use std::collections::HashMap;

pub type Text<'a> = &'a str;
pub type MaybeName<'a> = Option<Text<'a>>;
pub type Seq<'a> = Vec<TextValue<'a>>;

#[derive(Clone, Debug, PartialEq)]
pub struct TextValue<'a> {
    pub name: MaybeName<'a>,
    pub value: Value<'a>,
    pub position: SrcPosition,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SrcPosition(pub usize);

#[derive(Clone, Debug, PartialEq)]
pub enum Value<'a> {
    // 3 basic values (those need to be supported by the text format)
    Text(Text<'a>),
    Seq(Seq<'a>),
    Maybe(Option<&'a TextValue<'a>>),
    // these are optional formats (that can be supported by the text
    // format - but are not required to)
    Bool(bool),
    U64(u64),
    I64(i64),
    F64(f64),
}

impl<'a> Value<'a> {
    pub fn is_none(&self) -> bool {
        if let Value::Maybe(maybe) = self {
            maybe.is_none()
        } else {
            false
        }
    }
}

impl<'a> From<Value<'a>> for TextValue<'a> {
    fn from(value: Value<'a>) -> Self {
        Self {
            name: Option::None,
            value,
            position: SrcPosition(0),
        }
    }
}

/// Naming:
/// - require: Returns an error when conversion is not possible.
/// - to: converts to some optional.
pub trait Converter {
    /// Converts an identifier to a string suited for the text format.
    fn identifier_to_string(identifier: &Identifier, what_for: IdentifierType) -> String {
        match what_for {
            IdentifierType::StructField => identifier.to_string(Format::SnakeCase)
        }        
    }

    fn to_text<'a>(value: &Value<'a>) -> Option<Text<'a>> {
        match value {
            Value::Text(value) => Option::Some(value),
            _ => Option::None,
        }
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

    fn require_maybe_present<'a>(value: &'a Value<'a>) -> Result<&'a TextValue<'a>, ParseError> {
        if let Value::Maybe(value) = value {
            if let Some(value) = value {
                Result::Ok(value)
            } else {
                Result::Err(ParseError::new(format!(
                    "Expecting a present value, got an absent value ({:?}).",
                    value
                )))
            }
        } else {
            Result::Err(ParseError::new(format!(
                "Expecting a maybe value, got({:?}).",
                value
            )))
        }
    }

    fn require_no_name(value: &TextValue) -> Result<(), ParseError> {
        if let Some(name) = value.name {
            Result::Err(ParseError::new(format!(
                "The given value has a name (name \
                 is {:?}). This name is unused and must be removed. Value is {:?}.",
                name, value
            )))
        } else {
            Result::Ok(())
        }
    }
}

pub enum IdentifierType {
    StructField,
}

fn require<T, Msg: FnOnce() -> String>(value: Option<T>, msg: Msg) -> Result<T, ParseError> {
    if let Some(value) = value {
        Result::Ok(value)
    } else {
        Result::Err(ParseError::new(msg()))
    }
}
