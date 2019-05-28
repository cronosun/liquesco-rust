use crate::value::Value;
use std::fmt::{Display, Error, Formatter};
use std::ops::Deref;

impl<'a> Display for Value<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Value::Bool(value) => {
                if *value {
                    write!(f, "Bool(true)")
                } else {
                    write!(f, "Bool(false)")
                }
            }
            Value::Unicode(value) => write!(f, "Text(\"{}\")", value),
            Value::Binary(value) => write!(f, "Binary({}, {:?})", value.len(), value),
            Value::Option(value) => {
                if let Some(value) = value {
                    write!(f, "Option({})", value.deref())
                } else {
                    write!(f, "Option(None)")
                }
            }
            Value::Seq(value) => {
                let len = value.len();
                if len == 0 {
                    write!(f, "Seq(empty)")
                } else {
                    write!(f, "Seq(len={}, [", len)?;
                    for item in value.deref() {
                        write!(f, "{}; ", item)?;
                    }
                    write!(f, "])")
                }
            }
            Value::Enum(value) => {
                let number_of_values = value.values.len();
                if number_of_values == 0 {
                    write!(f, "Enum(ordinal={})", value.ordinal)
                } else {
                    write!(f, "Enum(ordinal={}, [", value.ordinal)?;
                    for item in value.values.deref() {
                        write!(f, "{}; ", item)?;
                    }
                    write!(f, "])")
                }
            }
            Value::UInt(value) => write!(f, "UInt({})", value),
            Value::SInt(value) => write!(f, "SInt({})", value),
            Value::Float(value) => write!(f, "Float({})", value),
        }
    }
}
