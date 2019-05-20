use std::borrow::Cow;

pub type Text<'a> = Cow<'a, str>;
pub type MaybeName<'a> = Option<Text<'a>>;
pub type Seq<'a> = Vec<TextValue<'a>>;

#[derive(Clone, Debug, PartialEq)]
pub struct TextValue<'a> {
    pub name: MaybeName<'a>,
    pub value: Value<'a>,
    pub position: Option<SrcPosition>,
}

impl<'a> AsRef<Value<'a>> for TextValue<'a> {
    fn as_ref(&self) -> &Value<'a> {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SrcPosition(pub usize);

#[derive(Clone, Debug, PartialEq)]
pub enum Value<'a> {
    // 3 basic values (those need to be supported by the text format)
    Text(Text<'a>),
    Seq(Seq<'a>),
    Nothing,
    // these are optional formats (that can be supported by the text
    // format - but are not required to)
    Bool(bool),
    U64(u64),
    I64(i64),
    F64(f64),
}

impl<'a> Value<'a> {
    pub fn is_nothing(&self) -> bool {
        self == &Value::Nothing
    }
}

impl<'a> From<Value<'a>> for TextValue<'a> {
    fn from(value: Value<'a>) -> Self {
        Self {
            name: Option::None,
            value,
            position: Option::None,
        }
    }
}
