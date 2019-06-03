use crate::type_info::TypeInfo;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::{TypeRef, TypeContainer};
use liquesco_schema::schema_builder::SchemaBuilder;
use std::collections::HashMap;
use liquesco_common::error::LqError;

pub trait SchemaReader : TypeContainer {

    fn non_root_type_info<'a>(&'a self, reference: &'a TypeRef) -> Result<TypeInfo<'a>, LqError> {
        let any_type = self.require_type(reference)?;
        Ok(TypeInfo {
            any_type,
            reference : Some(reference),
            id: self.identifier(reference)
        })
    }

    fn root_type_info(&self) -> TypeInfo {
        let any_type = self.root();
        TypeInfo {
            any_type,
            reference : None,
            id: None
        }
    }

    fn type_info<'a>(&'a self, reference: Option<&'a TypeRef>) -> Result<TypeInfo<'a>, LqError> {
        if let Some(reference) = reference {
            self.non_root_type_info(reference)
        } else {
            Ok(self.root_type_info())
        }
    }

}

impl SchemaReader for TypeContainer {
}