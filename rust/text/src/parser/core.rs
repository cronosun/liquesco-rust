use crate::parser::value::TextValue;
use liquesco_core::schema::core::TypeRef;
use liquesco_core::common::error::LqError;
use liquesco_core::schema::core::Schema;
use liquesco_core::schema::core::Type;
use liquesco_core::serialization::core::LqWriter;
use crate::parser::converter::Converter;
use crate::parser::value::Value;
use crate::parser::value::SrcPosition;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::Display;
use std::num::TryFromIntError;

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

#[derive(Debug)]
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

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParseError({:?})", self)
    }
}

impl From<TryFromIntError> for ParseError {
    fn from(value: TryFromIntError) -> Self {
        let lq_error : LqError = value.into();
        lq_error.into()
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
