use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::TypeRef;
use liquesco_schema::identifier::{Format, Identifier};
use std::borrow::Cow;

pub struct TypeInfo<'a> {
    any_type: &'a AnyType<'a>,
    reference: &'a TypeRef,
    id: Cow<'a, Identifier<'a>>,
}

impl<'a> TypeInfo<'a> {
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
