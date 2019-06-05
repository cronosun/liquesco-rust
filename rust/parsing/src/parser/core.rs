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
    
    fn parse(
        &mut self,
        writer: &mut Self::TWriter,
        r#type: &TypeRef,
        value: &TextValue,
    ) -> Result<(), LqError>;

    fn parse_to_vec(
        &mut self,
         r#type: &TypeRef,
        value: &TextValue) -> Result<Vec<u8>, LqError>;

    // TODO: Remove
    fn anchor_info(&mut self) -> &mut Option<AnchorInfo>;

// TODO: Remove
    fn take_anchor_info(&mut self) -> Option<AnchorInfo>;

// TODO: Remove
    fn set_anchor_info(&mut self, anchor_info: Option<AnchorInfo>);

// TODO: Remove
    fn present_anchor_info(&mut self) -> &mut AnchorInfo;

    fn remove_anchors(&mut self) -> Option<HashMap<Vec<u8>, u32>> {
        unimplemented!()
    }

    fn set_anchors(&mut self, anchors : Option<HashMap<Vec<u8>, u32>>) {
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

// TODO: Remove
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
