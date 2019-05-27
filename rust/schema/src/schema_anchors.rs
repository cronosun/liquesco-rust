use crate::anchors::TAnchors;
use crate::any_type::AnyType;
use crate::core::TypeContainer;
use crate::core::TypeRef;
use crate::metadata::MetadataSetter;
use crate::metadata::NameDescription;
use crate::schema_builder::BuildsOwnSchema;
use crate::schema_builder::SchemaBuilder;
use liquesco_common::error::LqError;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryFrom;

/// This is the base of a liquesco schema.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SchemaAnchors<'a> {
    pub r#type: Cow<'a, AnyType<'a>>,
    pub types: Cow<'a, [AnyType<'a>]>,
}

impl SchemaAnchors<'_> {
    pub fn main_type(&self) -> TypeRef {
        TypeRef(u32::try_from(self.types.len()).unwrap())
    }
}

impl<'a> TypeContainer<'a> for SchemaAnchors<'a> {
    fn maybe_type(&self, reference: TypeRef) -> Option<&AnyType<'a>> {
        let ref_num = reference.0;
        if ref_num == 0 {
            Some(&self.r#type)
        } else {
            let number_of_anchors = self.types.len();
            let usize_ref_num: Result<usize, _> = TryFrom::try_from(ref_num);
            if let Some(usize_ref_num) = usize_ref_num.ok() {
                let index = usize_ref_num - 1;
                if number_of_anchors > index {
                    Some(&self.types[index])
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

impl BuildsOwnSchema for SchemaAnchors<'_> {
    fn build_schema<B>(builder: &mut B) -> TypeRef
    where
        B: SchemaBuilder,
    {
        let any_type = AnyType::build_schema(builder);
        builder.add(
            TAnchors::new(any_type, any_type).with_meta(NameDescription {
                name: "schema_anchors",
                description:
                    "This anchors is the base of any liquesco schema. It \
                     contains at least one type. It can contain more types that can be referenced \
                     to create recursive types.",
            }),
        )
    }
}

pub struct SchemaAnchorsBuilder<'a> {
    type_to_ref: HashMap<AnyType<'a>, TypeRef>,
}

impl<'a> Default for SchemaAnchorsBuilder<'a> {
    fn default() -> Self {
        Self {
            type_to_ref: HashMap::default(),
        }
    }
}

impl<'a> SchemaBuilder for SchemaAnchorsBuilder<'a> {
    fn add<T: Into<AnyType<'static>>>(&mut self, item: T) -> TypeRef {
        let any_type = item.into();
        // already has a type id?
        if let Some(existing_ref) = self.type_to_ref.get(&any_type) {
            *existing_ref
        } else {
            // create a new one
            let len = self.type_to_ref.len();
            let reference = TypeRef(u32::try_from(len).unwrap());
            self.type_to_ref.insert(any_type, reference);
            reference
        }
    }
}

impl<'a> TryFrom<SchemaAnchorsBuilder<'a>> for SchemaAnchors<'a> {
    type Error = LqError;

    fn try_from(value: SchemaAnchorsBuilder<'a>) -> Result<Self, Self::Error> {
        let len = value.type_to_ref.len();
        if len == 0 {
            return LqError::err_new(
                "Have no anchors (at least the master anchor is \
                 required).",
            );
        }
        let len_without_main = len - 1;
        let main_ref = TypeRef(u32::try_from(len_without_main)?);

        let mut main: Option<AnyType<'a>> = None;
        let mut vec: Vec<Option<AnyType<'a>>> = Vec::with_capacity(len_without_main);
        for _ in 0..len_without_main {
            vec.push(None);
        }
        for entry in value.type_to_ref {
            if entry.1 == main_ref {
                main = Some(entry.0);
            } else {
                let index = usize::try_from((entry.1).0)?;
                vec.remove(index);
                vec.insert(index, Some(entry.0));
            }
        }

        let resulting_vec: Vec<AnyType<'a>> = vec.into_iter().map(|item| item.unwrap()).collect();

        Ok(SchemaAnchors {
            r#type: Cow::Owned(main.unwrap()),
            types: Cow::Owned(resulting_vec),
        })
    }
}
