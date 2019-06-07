use crate::parser::converter::Converter;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::core::Schema;
use liquesco_schema::core::Type;
use liquesco_schema::core::TypeRef;
use liquesco_serialization::core::LqWriter;
use std::collections::HashMap;

pub trait Context<'a> {
    type TConverter: Converter;
    type TSchema: Schema;
    type TWriter: LqWriter;

    fn schema(&self) -> &Self::TSchema;

    // TODO: Versuchen das &mut self, nach &self zu machen (müsste eigentlich gehen) -> In diesem fall könnte man das clonen in 'PKeyRef' verhindern. Vermutlich müsste man das AnchorInfo anpassen, man könnte da pointer nehmen die zurück verweisen
    fn parse(
        &mut self,
        writer: &mut Self::TWriter,
        r#type: &TypeRef,
        value: &TextValue,
    ) -> Result<(), LqError>;

    // TODO: The same : &mut -> &
    fn parse_to_vec(
        &mut self,
         r#type: &TypeRef,
        value: &TextValue) -> Result<Vec<u8>, LqError>;

    fn push_anchors(&mut self, anchors : AnchorInfo) {
        unimplemented!()
    }

    /// Pops the anchors info from top of the stack. Returns error if stack is empty.
    fn pop_anchors(&mut self) -> Result<(), LqError> {
        unimplemented!()
    }

    /// Returns the anchor info or empty if there's no anchor info.
    fn anchors(&self, level : u32) -> Option<&AnchorInfo> {
        unimplemented!()
    }
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
    ) -> Result<(), LqError>
    where
        C: Context<'c>;
}

#[derive(Debug)]
pub struct AnchorInfo {
    /// The serialized anchors mapped to index
    anchors: HashMap<Vec<u8>, u32>,
    key_type : TypeRef,
}

impl AnchorInfo {
    pub fn new(anchors: HashMap<Vec<u8>, u32>,
               key_type : TypeRef) -> Self {
        AnchorInfo {
            anchors, r#key_type
        }
    }

    pub fn key_type(&self) -> &TypeRef {
        &self.key_type
    }

    pub fn anchors(&self) -> &HashMap<Vec<u8>, u32> {
        &self.anchors
    }
}
