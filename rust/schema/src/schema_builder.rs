use crate::any_type::AnyType;
use crate::core::TypeContainer;
use crate::core::{Type, TypeRef};
use crate::identifier::Identifier;
use crate::identifier::StrIdentifier;
use crate::structure::TStruct;
use crate::type_container::DefaultTypeContainer;
use liquesco_common::error::LqError;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::convert::TryFrom;

pub trait SchemaBuilder<'a> {
    type TTypeContainer: TypeContainer<'a>;

    /// Adds a type to the schema. What happens on duplicate IDs? This depends on the
    /// implementation: Might return an error or might adjust the ID (that's why we return
    /// the type ref).
    fn add<T: Into<AnyType<'a>>>(
        &mut self,
        id: StrIdentifier<'static>,
        item: T,
    ) -> Result<TypeRef, LqError>;

    fn add_unwrap<T: Into<AnyType<'a>>>(&mut self, id: &'static str, item: T) -> TypeRef {
        let identifier = StrIdentifier::try_from(Cow::Borrowed(id)).unwrap();
        self.add(identifier, item).unwrap()
    }

    fn finish<T: Into<AnyType<'a>>>(self, root: T) -> Result<Self::TTypeContainer, LqError>;
}

/// Something that can build its own schema.
pub trait BuildsOwnSchema {
    fn build_schema<B>(builder: &mut B) -> TypeRef
    where
        B: SchemaBuilder<'static>;
}

/// Something that builds its own schema and is the root type.
pub trait RootBuildsOwnSchema {
    fn root_build_schema<B>(builder: B) -> Result<B::TTypeContainer, LqError>
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
    types: BTreeMap<StrIdentifier<'static>, AnyType<'a>>,
}

impl<'a> Default for DefaultSchemaBuilder<'a> {
    fn default() -> Self {
        Self {
            types: BTreeMap::new(),
        }
    }
}

impl<'a> SchemaBuilder<'a> for DefaultSchemaBuilder<'a> {
    type TTypeContainer = DefaultTypeContainer<'a>;

    fn add<T: Into<AnyType<'a>>>(
        &mut self,
        id: StrIdentifier<'static>,
        item: T,
    ) -> Result<TypeRef, LqError> {
        let any_type = item.into();

        // make sure we don't store different types with same ID
        if let Some(existing) = self.types.remove(&id) {
            if existing != any_type {
                return LqError::err_new(format!(
                    "You're trying to add different types with \
                     the same ID {:?}. Type A is {:?}, type B is {:?}.",
                    id, existing, any_type
                ));
            }
        }

        self.types.insert(id.clone(), any_type);

        Ok(TypeRef::Identifier(id))
    }

    fn finish<T: Into<AnyType<'a>>>(self, root: T) -> Result<Self::TTypeContainer, LqError> {
        let len = self.types.len();

        // First collect indexes / decompose types
        let mut index_map = HashMap::<StrIdentifier<'static>, u32>::with_capacity(len);
        let mut types_vec = Vec::<(Identifier<'a>, AnyType<'a>)>::with_capacity(len);
        for (index, (id, any_type)) in self.types.into_iter().enumerate() {
            let index_u32 = index as u32;
            index_map.insert(id.clone(), index_u32);
            let identifier: Identifier<'a> = id.into();
            types_vec.push((identifier, any_type));
        }

        // Now mutate all values: convert all string references to numerical references
        let mut types: Vec<(Identifier<'a>, AnyType<'a>)> = Vec::with_capacity(len);
        for (identifier, mut any_type) in types_vec.into_iter() {
            convert_type_refs(&mut any_type, &index_map)?;
            // Now add the "fixed" type to resulting map
            types.push((identifier, any_type));
        }

        // Also convert the root type
        let mut root_any = root.into();
        convert_type_refs(&mut root_any, &index_map)?;

        Ok(DefaultTypeContainer::new(types, root_any))
    }
}

fn convert_type_refs<'a>(
    any_type: &mut AnyType<'a>,
    index_map: &HashMap<StrIdentifier<'static>, u32>,
) -> Result<(), LqError> {
    let mut ref_index = 0;
    while let Some(reference) = any_type.reference(ref_index) {
        if let TypeRef::Identifier(str_identifier) = reference {
            let index = index_map.get(str_identifier);
            if let Some(index) = index {
                any_type.set_reference(ref_index, TypeRef::new_numerical(*index))?;
            } else {
                // this should never happen
                return LqError::err_new(format!(
                    "Type {:?} not found in \
                     schema builder.",
                    str_identifier
                ));
            }
        }
        ref_index += 1;
    }
    Ok(())
}
