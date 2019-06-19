use liquesco_common::error::LqError;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::TypeContainer;
use liquesco_schema::core::TypeRef;
use liquesco_schema::identifier::Identifier;
use std::borrow::Cow;

#[derive(Clone)]
pub struct TypeInfo<'a> {
    any_type: &'a AnyType<'a>,
    reference: &'a TypeRef,
    id: Cow<'a, Identifier<'a>>,
}

impl<'a> TypeInfo<'a> {
    pub fn try_from(
        type_container: &'a TypeContainer,
        reference: &'a TypeRef,
    ) -> Result<Self, LqError> {
        let any_type = type_container.require_type(reference)?;
        Ok(Self {
            any_type,
            reference,
            id: type_container.identifier(reference)?,
        })
    }

    pub fn new(
        any_type: &'a AnyType<'a>,
        reference: &'a TypeRef,
        id: Cow<'a, Identifier<'a>>,
    ) -> Self {
        Self {
            any_type,
            reference,
            id,
        }
    }

    pub fn any_type(&self) -> &AnyType {
        self.any_type
    }

    pub fn reference(&self) -> &TypeRef {
        self.reference
    }

    pub fn identifier(&self) -> &Identifier {
        &self.id
    }
}
