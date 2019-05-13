use crate::value::TextValue;
use liquesco_core::schema::core::TypeRef;
use liquesco_core::common::error::LqError;
use liquesco_core::schema::core::Schema;
use liquesco_core::schema::core::Type;
use liquesco_core::serialization::core::LqWriter;
use crate::value::Converter;
use crate::value::Value;
use crate::value::SrcPosition;
use std::borrow::Cow;

pub trait Context<'a> {
    type TConverter: Converter;
    type TSchema: Schema<'a>;
    type TWriter: LqWriter;
    
    fn schema(&self) -> &Self::TSchema;
    fn value(&self) -> &Value;
    fn text_value(&self) -> &TextValue;
    fn parse(&self, writer : &mut Self::TWriter, r#type : TypeRef, value : &TextValue) -> Result<(), ParseError>;
}

pub trait Parser<'a> {
    type T: Type<'a>;

    /// Parse the given value. Note: There's no need to do validation here (validation will be performed when 
    /// entire data has been written) - when the given value can be parsed it's sufficient.
    fn parse<'c, C>(context: &C, writer : &mut C::TWriter, r#type: &Self::T) -> Result<(), ParseError>
    where
        C: Context<'c>;
}

pub struct ParseError {
    msg: Option<Cow<'static, str>>,
    lq_error: Option<LqError>,
    src_position : Option<SrcPosition>,
}

impl From<LqError> for ParseError {
    fn from(value: LqError) -> Self {
        Self {
            msg: Option::None,
            lq_error: Option::Some(value),
            src_position : Option::None
        }
    }
}

impl ParseError {

    pub fn new<Msg : Into<Cow<'static, str>>>(msg : Msg) -> Self {
        ParseError {
            msg : Option::Some(msg.into()),
            lq_error : Option::None,
            src_position : Option::None,
        }
    }

    pub fn with_position<Pos : Into<SrcPosition>>(mut self, position : Pos) -> Self {
        self.src_position = Option::Some(position.into());
        self
    }
}
