use serde::{Deserialize, Serialize};

use crate::any_type::AnyType;
use crate::core::{TypeContainer, Type};
use crate::core::TypeRef;
use crate::identifier::Identifier;
use liquesco_common::error::LqError;
use std::borrow::Cow;
use std::hash::{Hasher, Hash};
use crate::metadata::{Information, WithMetadata, MetadataSetter};
use liquesco_serialization::serde::serialize_to_vec;
use std::convert::TryInto;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::type_hash::{TypeHash, TypeHasher};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefaultTypeContainer<'a> {
    types: Vec<(Identifier<'a>, AnyType<'a>)>,
    root: TypeRef,
    #[serde(skip_serializing, skip_deserializing)]
    cache: RefCell<HashCache>,
}

impl PartialEq for DefaultTypeContainer<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.types == other.types
    }
}

impl Eq for DefaultTypeContainer<'_> {}

impl<'a> DefaultTypeContainer<'a> {
    pub(crate) fn new(types: Vec<(Identifier<'a>, AnyType<'a>)>, root: TypeRef) -> Self {
        Self {
            types,
            root,
            cache: RefCell::new(HashCache::default()),
        }
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

    fn root(&self) -> &TypeRef {
        &self.root
    }

    fn identifier(&self, reference: &TypeRef) -> Result<Cow<Identifier>, LqError> {
        let maybe_id = match reference {
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
        };
        if let Some(id) = maybe_id {
            Ok(id)
        } else {
            LqError::err_new(format!("Type {:?} not found in schema.", reference))
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

    fn hash_type<H: Hasher>(&self, reference: &TypeRef,
                            information: Information, state: &mut H) -> Result<(), LqError> {
        let any_type = self.require_type(reference)?;
        let vec = if let Some(reduced_metadata) = any_type.meta().reduce_information(information) {
            let mut cloned_any = any_type.clone();
            cloned_any.set_meta(reduced_metadata);
            serialize_to_vec(&cloned_any)?
        } else {
            serialize_to_vec(any_type)?
        };

        vec.hash(state);

        // Do the same for all dependencies
        let mut index = 0;
        while let Some(reference) = any_type.reference(index) {
            self.hash_type(reference, information, state)?;
            index = index + 1;
        }

        // write number of dependencies as u64
        let number_of_dependencies: u64 = index.try_into()?;
        number_of_dependencies.hash(state);

        Ok(())
    }

    fn type_hash(&self, reference: &TypeRef, information: Information) -> Result<TypeHash, LqError> {
        let entry = CacheEntry {
            // Note: Clone should be cheap, since we have the numerical version here
            reference: reference.clone(),
            information,
        };

        {
            if let Some(result) = self.cache.borrow().entries.get(&entry) {
                return Ok(result.clone());
            }
        }

        // not in cache: generate hash and add to cache
        let mut hasher = TypeHasher::default();
        self.hash_type(reference, information, &mut hasher)?;
        let type_hash = hasher.finish();

        {
            match self.cache.try_borrow_mut() {
                Ok(mut cache) => {
                    cache.entries.insert(entry, type_hash);
                    Ok(())
                }
                Err(err) => {
                    Err(LqError::new(format!("Unable to borrow type hash cache: {:?}", err)))
                }
            }?;
        }
        // and try again
        self.type_hash(reference, information)
    }
}

#[derive(Clone, Debug)]
struct HashCache {
    entries: HashMap<CacheEntry, TypeHash>
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct CacheEntry {
    reference: TypeRef,
    information: Information,
}

impl Default for HashCache {
    fn default() -> Self {
        Self {
            entries: HashMap::default()
        }
    }
}