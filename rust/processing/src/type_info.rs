use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::TypeRef;
use liquesco_schema::identifier::{Identifier, Format};
use std::borrow::Cow;

/// TODO: Make getters & private
pub struct TypeInfo<'a> {
    pub any_type: &'a AnyType<'a>,
    /// This is empty if it's the root type.
    pub reference: Option<&'a TypeRef>,
    /// This is empty if it's the root type.
    pub id : Option<Cow<'a, Identifier<'a>>>,
}

impl<'a> TypeInfo<'a> {

    pub fn any_type(&self) -> &AnyType {
        self.any_type
    }

    pub fn reference(&self) -> Option<&TypeRef> {
        self.reference
    }

}