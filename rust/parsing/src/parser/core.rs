use crate::parser::converter::Converter;
use crate::parser::value::SrcPosition;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::core::Schema;
use liquesco_schema::core::Type;
use liquesco_schema::core::TypeRef;
use liquesco_serialization::core::LqWriter;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::num::TryFromIntError;

pub trait Context<'a> {
    type TConverter: Converter;
    type TSchema: Schema<'a>;
    type TWriter: LqWriter;

    fn schema(&self) -> &Self::TSchema;
    fn parse(
        &mut self,
        writer: &mut Self::TWriter,
        r#type: TypeRef,
        value: &TextValue,
    ) -> Result<(), ParseError>;

    fn anchor_info(&mut self) -> &mut Option<AnchorInfo>;

    fn take_anchor_info(&mut self) -> Option<AnchorInfo>;

    fn set_anchor_info(&mut self, anchor_info: Option<AnchorInfo>);

    fn present_anchor_info(&mut self) -> &mut AnchorInfo;
}

pub trait Parser<'a> {
    type T: Type;

    /// Parse the given value. Note: There's no need to do validation here (validation will be performed when
    /// entire data has been written) - when the given value can be parsed it's sufficient.
    fn parse<'c, C>(
        context: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        r#type: &Self::T,
    ) -> Result<(), ParseError>
    where
        C: Context<'c>;
}

#[derive(Debug)]
pub struct ParseError {
    msg: Option<Cow<'static, str>>,
    lq_error: Option<LqError>,
    src_position: Option<SrcPosition>,
}

impl From<LqError> for ParseError {
    fn from(value: LqError) -> Self {
        Self {
            msg: Option::None,
            lq_error: Option::Some(value),
            src_position: Option::None,
        }
    }
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParseError({:?})", self)
    }
}

impl From<TryFromIntError> for ParseError {
    fn from(value: TryFromIntError) -> Self {
        let lq_error: LqError = value.into();
        lq_error.into()
    }
}

impl ParseError {
    pub fn new<Msg: Into<Cow<'static, str>>>(msg: Msg) -> Self {
        ParseError {
            msg: Option::Some(msg.into()),
            lq_error: Option::None,
            src_position: Option::None,
        }
    }

    pub fn with_position<Pos: Into<SrcPosition>>(mut self, position: Pos) -> Self {
        self.src_position = Option::Some(position.into());
        self
    }
}

#[derive(Debug)]
pub struct AnchorInfo {
    anchors: HashMap<String, u32>,
    anchors_by_index: Vec<String>,
}

impl Default for AnchorInfo {
    fn default() -> Self {
        Self {
            anchors: HashMap::default(),
            anchors_by_index: Vec::default(),
        }
    }
}

impl AnchorInfo {
    pub fn reference(&mut self, name: &str) -> u32 {
        if let Some(found) = self.anchors.get(name) {
            *found
        } else {
            let len = self.anchors.len();
            let len_u32 = len as u32;
            self.anchors.insert(name.to_string(), len_u32);
            self.anchors_by_index.push(name.to_string());
            len_u32
        }
    }

    pub fn by_index(&self, index: u32) -> Option<&str> {
        self.anchors_by_index
            .get(index as usize)
            .map(|string| string.as_str())
    }
}
