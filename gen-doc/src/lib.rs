#[macro_use]
extern crate lazy_static;

use std::any::TypeId;
use liquesco_schema::core::{TypeRef, TypeContainer};
use crate::model::card::CardId;
use crate::model::Model;
use liquesco_common::error::LqError;
use crate::model_writer::ModelWriter;

pub mod model;
pub mod context;
pub mod types;
pub mod usage;
pub mod type_writer;
pub mod type_parts;
pub mod type_description;
pub mod model_writer;

pub fn create_model(schema: &TypeContainer) -> Result<impl Model, LqError> {
    let mut writer = ModelWriter::new(schema);
    writer.process(schema.root())
}

impl From<&TypeRef> for CardId {
    fn from(reference: &TypeRef) -> Self {
        match reference {
            TypeRef::Numerical(num) => {
                CardId::new(format!("num:{}", num))
            },
            TypeRef::Identifier(id) => {
                CardId::new(format!("id:{}", id.as_string()))
            }
        }
    }
}