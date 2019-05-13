use crate::text::parser::ParseError;
use std::collections::HashMap;
use std::rc::Rc;

pub type RcText = Rc<str>;
pub type MaybeName = Option<RcText>;
pub type RcSeq = Rc<Vec<TextValue>>;
pub type RcMap = Rc<HashMap<RcText, TextValue>>;

#[derive(Clone, Debug)]
pub struct TextValue {
    pub name: MaybeName,
    pub value: Value,
    pub position: SrcPosition,
}

#[derive(Clone, Debug)]
pub struct SrcPosition(pub usize);

#[derive(Clone, Debug)]
pub enum Value {
    // 3 basic values (those need to be supported by the text format)
    Text(RcText),
    Seq(RcSeq),
    Nothing,
    // these are optional formats (that can be supported by the text
    // format - but are not required to)
    Bool(bool),
    U64(u64),
    I64(i64),
    F64(f64),
}

pub trait Converter {
    fn into_text(value: Value) -> Option<RcText> {
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

    fn require_u64(value: &Value) -> Result<u64, ParseError> {
        // todo
        Result::Ok(Self::to_u64(value).unwrap())
    }

}
