use serde::{Deserialize, Serialize};

use crate::any_type::AnyType;
use crate::core::TypeContainer;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use liquesco_common::error::LqError;

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

impl<'a> TypeContainer<'a> for DefaultTypeContainer<'a> {
    fn maybe_type(&self, reference: &TypeRef) -> Option<&AnyType<'a>> {
        match reference {
            TypeRef::Numerical(num) => {
                // Note: This does not reference the key, it references the index (but this
                // should be more or less the same)
                self.types.get(*num as usize).map(|entry| &entry.1)
            }
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

    fn root(&self) -> &AnyType<'a> {
        &self.root
    }

    fn require_type(&self, reference: &TypeRef) -> Result<&AnyType<'a>, LqError> {
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
