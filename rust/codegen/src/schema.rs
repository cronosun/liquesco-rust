use liquesco_common::error::LqError;
use std::collections::HashSet;
use liquesco_schema::identifier::Identifier;
use liquesco_schema::identifier::Segment;
use std::collections::HashMap;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::TypeRef;
use std::convert::TryFrom;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::hash::Hash;

pub trait SchemaReader {
    fn master_ref(&self) -> TypeRef;
    fn require(&self, reference : TypeRef) -> &AnyType;
}

/// Generates (or just supplies) names for schema types.
pub struct NameSupplier {
    used_names : HashMap<Identifier<'static>, usize>,
    name_for : HashMap<TypeRef, Identifier<'static>>
}

impl NameSupplier {
    pub fn technical_name_for<R>(&mut self, reader : &R, type_ref : TypeRef) -> &Identifier where R: SchemaReader {
        if let Some(name) = self.name_for.get(&type_ref) {
            // best case (we already have a name)
            name
        } else {
            let any_type = reader.require(type_ref);
            if let Some(name) = any_type.doc().name() {
                // yes, type has a given name
                if let Some(number_used) = self.used_names.get(name) {
                    // we have a problem (duplicate)
                    let alt_name = Self::generate_alt_name(any_type, name, *number_used);
                    // increase number used
                    self.used_names.insert(name.into_owned(), number_used + 1);
                    self.name_for.insert(type_ref, alt_name.into_owned());
                    self.name_for.get(type_ref).unwrap()
                } else {
                    // mark that we used the name
                    self.used_names.insert(name.clone(), 1);
                    self.name_for.insert(type_ref, name.clone());
                    name
                }
            }
        }
    }

    pub fn hash(any_type : &AnyType) -> u64 {
        let mut hasher = DefaultHasher::new();
        any_type.hash(&mut hasher);
        hasher.finish()
    }

    pub fn hash_to_identifier(any_type : &AnyType) -> Identifier<'static> {
        Identifier::try_from(format!("type_{:?}", Self::hash(any_type))).unwrap()
    }

    fn maybe_generate_alt_name(any_type : &AnyType, name : &Identifier, num_used : usize) -> Result<Identifier<'static>, LqError> {
        let mut new_name : Identifier<'static> = name.into_owned();
        let segment = Segment::try_from(format!("{:?}", num_used))?;
        new_name.append(segment)?;
        Ok(new_name)
    }

    fn generate_alt_name(any_type : &AnyType, name : &Identifier, num_used : usize) -> Identifier<'static> {
        if let Ok(alt_name) = Self::generate_alt_name(name, num_used) {
            alt_name
        } else {
            // special case: this can happen when we can't extend the Identifier
            hash_to_identifier(any_type)
        }
    }
}
