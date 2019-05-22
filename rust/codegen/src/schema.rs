use liquesco_common::error::LqError;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::TypeRef;
use liquesco_schema::identifier::Identifier;
use liquesco_schema::identifier::Segment;
use liquesco_schema::schema_builder::SchemaBuilder;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::Hash;
use std::hash::Hasher;

pub trait SchemaReader {
    fn master_ref(&self) -> TypeRef;
    fn require(&self, reference: TypeRef) -> &AnyType;
}

/// Generates (or just supplies) names for schema types.
pub struct NameSupplier {
    // used to avoid conflicts
    used_names: HashMap<Identifier<'static>, usize>,
    name_for: HashMap<TypeRef, Identifier<'static>>,
    hashed_display_name: HashMap<TypeRef, Identifier<'static>>,
}

impl Default for NameSupplier {
    fn default() -> Self {
        Self {
            used_names: HashMap::default(),
            name_for: HashMap::default(),
            hashed_display_name: HashMap::default(),
        }
    }
}

impl NameSupplier {
    /// Generates the technical name for the type. It's a unique name.
    pub fn technical_name_for(&mut self, any_type : &AnyType, type_ref: TypeRef) -> &Identifier
    {
        if let None = self.name_for.get(&type_ref) {
            if let Some(name) = any_type.doc().name() {
                // yes, type has a given name
                if let Some(number_used) = self.used_names.get(name) {
                    // we have a problem (duplicate)
                    let alt_name = Self::generate_alt_name(any_type, name, *number_used);
                    // increase number used
                    self.used_names
                        .insert(name.clone().into_owned(), number_used + 1);
                    self.name_for.insert(type_ref, alt_name.into_owned());
                } else {
                    // mark that we used the name
                    self.used_names.insert(name.clone().into_owned(), 1);
                    self.name_for.insert(type_ref, name.clone().into_owned());
                }
            } else {
                // completely no name, so generate the name
                let hashed_name = Self::hash_to_identifier(any_type);
                self.used_names.insert(hashed_name.clone().into_owned(), 1);
                self.name_for.insert(type_ref, hashed_name.into_owned());
            }
        };

        self.name_for.get(&type_ref).unwrap()
    }

    /// Returns the display name for some given type.
    pub fn display_name_for<'a, 'b: 'a>(
        &'a mut self,
        type_ref: TypeRef,
        any_type: &'b AnyType<'b>,
    ) -> &'a Identifier<'a> {
        if let Some(name) = any_type.doc().name() {
            name
        } else {
            // ok, we have no name, take the hash name
            if let None = self.hashed_display_name.get(&type_ref) {
                let hashed_name = Self::hash_to_identifier(any_type);
                self.hashed_display_name.insert(type_ref, hashed_name);
            }
            self.hashed_display_name.get(&type_ref).unwrap()
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
}

/// It's at the same time a `SchemaBuilder` and a `SchemaReader`.
pub struct SchemaBuilderReader {
    type_to_ref: HashMap<AnyType<'static>, TypeRef>,
    reference_to_type: HashMap<TypeRef, AnyType<'static>>,
}

impl Default for SchemaBuilderReader {
    fn default() -> Self {
        SchemaBuilderReader {
            type_to_ref: HashMap::new(),
            reference_to_type: HashMap::new(),
        }
    }
}

impl SchemaBuilder for SchemaBuilderReader {
    fn add<T: Into<AnyType<'static>>>(&mut self, item: T) -> TypeRef {
        let any = item.into();
        // store items, this prevents duplicates
        if let Some(reference) = self.type_to_ref.get(&any) {
            *reference
        } else {
            // not yet in map / no duplicate
            let new_reference = TypeRef(self.type_to_ref.len() as u32);
            // TODO: Prevent cloning
            self.reference_to_type.insert(new_reference, any.clone());
            self.type_to_ref.insert(any, new_reference);
            new_reference
        }
    }
}

impl SchemaReader for SchemaBuilderReader {
    fn master_ref(&self) -> TypeRef {
        TypeRef(self.type_to_ref.len() as u32)
    }

    fn require(&self, reference: TypeRef) -> &AnyType {
        if let Some(any) = self.reference_to_type.get(&reference) {
            any
        } else {
            panic!(format!(
                "Type with reference {:?} not found in schema",
                reference
            ))
        }
    }
}
