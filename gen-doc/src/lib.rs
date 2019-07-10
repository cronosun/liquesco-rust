#[macro_use]
extern crate lazy_static;

use crate::model::card::CardId;
use crate::model::Model;
use crate::model_writer::ModelWriter;
use liquesco_common::error::LqError;
use liquesco_schema::core::{TypeContainer, TypeRef};
use liquesco_schema::schema::schema_schema;
use liquesco_schema::schema_builder::DefaultSchemaBuilder;
use std::convert::TryFrom;

pub mod adoc;
pub mod context;
pub mod model;
pub mod model_writer;
pub mod type_description;
pub mod type_parts;
pub mod type_writer;
pub mod types;
pub mod usage;

static CARD_ID_FROM_TYPE_REF: &str = "card_id_from_type_ref";

pub fn create_model(schema: &TypeContainer) -> Result<impl Model, LqError> {
    let writer = ModelWriter::new(schema);
    writer.process(schema.root())
}

pub fn create_model_from_schema_schema() -> Result<impl Model, LqError> {
    let builder = DefaultSchemaBuilder::default();
    let schema = schema_schema(builder).unwrap();
    let type_container: &TypeContainer = &schema;

    create_model(type_container)
}

impl TryFrom<&TypeRef> for CardId {
    type Error = LqError;

    fn try_from(value: &TypeRef) -> Result<Self, Self::Error> {
        match value {
            TypeRef::Numerical(num) => Ok(CardId::new(CARD_ID_FROM_TYPE_REF, *num as usize)),
            TypeRef::Identifier(_) => Err(LqError::new(format!(
                "Unable to \
                 convert identifier type ref to card id (can only convert identifier \
                 type refs): {:?}",
                value
            ))),
        }
    }
}
