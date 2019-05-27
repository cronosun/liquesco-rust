use crate::type_info::TypeInfo;
use liquesco_common::error::LqError;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::TypeRef;
use liquesco_schema::identifier::Identifier;
use liquesco_schema::identifier::Segment;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::Hash;
use std::hash::Hasher;
use liquesco_schema::metadata::WithMetadata;

/// Generates (or just supplies) names for schema types.
pub struct Names {
    // used to avoid conflicts
    used_names: HashMap<Identifier<'static>, usize>,
    name_for: HashMap<TypeRef, Identifier<'static>>,
    hashed_display_name: HashMap<TypeRef, Identifier<'static>>,
}

impl Default for Names {
    fn default() -> Self {
        Names {
            used_names: HashMap::default(),
            name_for: HashMap::default(),
            hashed_display_name: HashMap::default(),
        }
    }
}

impl Names {
    /// Generates the technical name for the type. It's a unique name.
    pub fn technical_name_for(&mut self, info: &TypeInfo) -> &Identifier {
        if let None = self.name_for.get(&info.reference) {
            self.create_and_insert_technical_name(info.any_type, info.reference);
        }

        // should be there now
        self.name_for.get(&info.reference).unwrap()
    }

    /// Returns the display name for some given type.
    pub fn display_name_for<'a, 'b: 'a>(
        &'a mut self,
        info: &'b TypeInfo<'b>,
    ) -> &'a Identifier<'a> {
        if let Some(name) = info.any_type.meta().name() {
            name
        } else {
            // ok, we have no name, take the hash name
            if let None = self.hashed_display_name.get(&info.reference) {
                let hashed_name = Self::hash_to_identifier(info.any_type);
                self.hashed_display_name.insert(info.reference, hashed_name);
            }
            self.hashed_display_name.get(&info.reference).unwrap()
        }
    }

    pub fn hash(any_type: &AnyType) -> u64 {
        let mut hasher = DefaultHasher::new();
        any_type.hash(&mut hasher);
        hasher.finish()
    }

    pub fn hash_to_identifier(any_type: &AnyType) -> Identifier<'static> {
        let string: &str = &format!("type_{:?}", Self::hash(any_type));
        Identifier::try_from(string).unwrap().into_owned()
    }

    fn maybe_generate_alt_name(
        _: &AnyType,
        name: &Identifier,
        num_used: usize,
    ) -> Result<Identifier<'static>, LqError> {
        let mut new_name: Identifier<'static> = name.clone().into_owned();
        let segment = Segment::try_from(format!("{:?}", num_used))?;
        new_name.append(segment)?;
        Ok(new_name)
    }

    fn generate_alt_name(
        any_type: &AnyType,
        name: &Identifier,
        num_used: usize,
    ) -> Identifier<'static> {
        if let Ok(alt_name) = Self::maybe_generate_alt_name(any_type, name, num_used) {
            alt_name
        } else {
            // special case: this can happen when we can't extend the Identifier
            Self::hash_to_identifier(any_type)
        }
    }

    fn create_and_insert_technical_name(&mut self, any_type: &AnyType, type_ref: TypeRef) {
        let created_name = self.create_technical_name(any_type);
        self.used_names.insert(created_name.clone().into_owned(), 1);
        self.name_for.insert(type_ref, created_name);
    }

    fn create_technical_name(&self, any_type: &AnyType) -> Identifier<'static> {
        if let Some(name) = any_type.meta().name() {
            // yes, type has a given name
            if let Some(number_used) = self.used_names.get(name) {
                // we have a problem (duplicate)
                Self::generate_alt_name(any_type, name, *number_used)
            } else {
                name.clone().into_owned()
            }
        } else {
            // completely no name, so generate the name
            Self::hash_to_identifier(any_type)
        }
    }
}
