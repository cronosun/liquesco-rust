use crate::any_type::AnyType;
use crate::core::TypeRef;
use crate::structure::TStruct;
use crate::identifier::Identifier;
use liquesco_common::error::LqError;
use std::collections::BTreeMap;
use std::convert::TryFrom;

pub trait SchemaBuilder<'a> {
    /// Adds a type to the schema. What happens on duplicate IDs? This depends on the
    /// implementation: Might return an error or might adjust the ID (that's why we return
    /// the type ref).
    fn add<T: Into<AnyType<'a>>>(&mut self, id : Identifier<'a>, item: T) -> Result<TypeRef /* TODO: Return Reference? */, LqError>;

    fn add_unwrap<T: Into<AnyType<'a>>>(&mut self, id : &'a str, item: T) -> TypeRef {
        let identifier = Identifier::try_from(id).unwrap();
        self.add(identifier, item).unwrap()
    }
}

/// Something that can build its own schema.
pub trait BuildsOwnSchema {
    fn build_schema<B>(builder: &mut B) -> TypeRef
    where
        B: SchemaBuilder<'static>;
}

/// A base type (a single type, not the any type) that can build its own schema. Note:
/// It's always a struct.
pub(crate) trait BaseTypeSchemaBuilder {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder<'static>;
}

/// Implementation of `SchemaBuilder` that returns errors when you try to add different any types
/// with the same ID.
pub struct DefaultSchemaBuilder<'a> {
    types : BTreeMap<Identifier<'a>, AnyType<'a>>,
    root : Option<AnyType<'a>>,
}

impl<'a> Default for DefaultSchemaBuilder<'a> {
    fn default() -> Self {
        Self {
            types : BTreeMap::new(),
            root : None
        }
    }
}

impl<'a> SchemaBuilder<'a> for DefaultSchemaBuilder<'a> {

    fn add<T: Into<AnyType<'a>>>(&mut self, id: Identifier<'a>, item: T) -> Result<TypeRef, LqError> {
        let any_type = item.into();

        // make sure we don't store different types with same ID
        if let Some(existing) = self.types.remove(&id) {
            if existing!=any_type {
                return LqError::err_new(format!("You're trying to add different types with \
                the same ID {}. Type A is {:?}, type B is {:?}.", id, existing, any_type))
            }
        }

        self.types.insert(id, any_type);

        Ok(TypeRef::Identifier(id.into_owned())) // TODO: Prevent cloning here
    }
}