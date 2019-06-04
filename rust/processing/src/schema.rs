use crate::type_info::TypeInfo;
use liquesco_common::error::LqError;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::{TypeContainer, TypeRef};
use liquesco_schema::schema_builder::SchemaBuilder;
use std::collections::HashMap;

pub trait SchemaReader: TypeContainer {
    fn type_info<'a>(&'a self, reference: &'a TypeRef) -> Result<TypeInfo<'a>, LqError> {
        let any_type = self.require_type(reference)?;
        Ok(TypeInfo::new(
            any_type,
            reference,
            self.identifier(reference)?,
        ))
    }
}

impl SchemaReader for TypeContainer {}
