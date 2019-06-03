use serde::{Deserialize, Serialize};

use crate::any_type::AnyType;
use crate::core::TypeContainer;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use liquesco_common::error::LqError;
use std::borrow::Cow;

#[derive(Clone, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct DefaultTypeContainer<'a> {
    types: Vec<(Identifier<'a>, AnyType<'a>)>,
    root: AnyType<'a>,
}

impl<'a> DefaultTypeContainer<'a> {
    pub(crate) fn new(types: Vec<(Identifier<'a>, AnyType<'a>)>, root: AnyType<'a>) -> Self {
        Self { types, root }
    }
}

impl<'a> TypeContainer for DefaultTypeContainer<'a> {
    fn maybe_type(&self, reference: &TypeRef) -> Option<&AnyType> {
        match reference {
            TypeRef::Numerical(num) => self.types.get(*num as usize).map(|entry| &entry.1),
            TypeRef::Identifier(string_id) => {
                // find by ID should usually not happen (since all types should have been converted)
                let id = string_id.clone().into();
                self.types
                    .iter()
                    .find(|item| item.0 == id)
                    .map(|item| &item.1)
            }
        }
    }

    fn root(&self) -> &AnyType {
        &self.root
    }

    fn identifier(&self, reference: &TypeRef) -> Option<Cow<Identifier>> {
        match reference {
            TypeRef::Numerical(num) => self
                .types
                .get(*num as usize)
                .map(|entry| Cow::Borrowed(&entry.0)),
            TypeRef::Identifier(string_id) => {
                // Should usually not be called
                let id_as_string = string_id.as_string();
                if let Ok(id) = Identifier::new_owned(id_as_string) {
                    Some(Cow::Owned(id))
                } else {
                    None
                }
            }
        }
    }

    fn require_type(&self, reference: &TypeRef) -> Result<&AnyType, LqError> {
        if let Some(present) = self.maybe_type(reference) {
            Ok(present)
        } else {
            let references: Vec<_> = self.types.iter().map(|entry| &entry.0).collect();
            LqError::err_new(format!(
                "There's no such type referenced by {}. All references \
                 I have: {:?}.",
                reference, references
            ))
        }
    }
}
