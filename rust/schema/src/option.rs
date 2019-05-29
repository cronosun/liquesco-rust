use crate::core::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::NameDescription;
use crate::metadata::WithMetadata;
use crate::reference::TReference;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::structure::Field;
use crate::structure::TStruct;
use liquesco_common::error::LqError;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::option::Presence;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

/// Data of the option type have two variants:
///  - Absent
///  - Present and a value
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TOption<'a> {
    meta: Meta<'a>,
    r#type: TypeRef,
}

impl TOption<'_> {

    /// The type of the present value.
    pub fn r#type(&self) -> TypeRef {
        self.r#type
    }

    /// Creates a new option type.
    pub fn new(r#type : TypeRef) -> Self {
        Self {
            meta : Meta::empty(),
            r#type
        }
    }
}

impl Type for TOption<'_> {

    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let presence = Presence::de_serialize(context.reader())?;

        match presence {
            Presence::Absent => Result::Ok(()),
            Presence::Present => context.validate(self.r#type),
        }
    }

    fn compare<'c, C>(
        &self,
        context: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>,
    {
        let presence1 = Presence::de_serialize(r1)?;
        let presence2 = Presence::de_serialize(r2)?;

        match (presence1, presence2) {
            (Presence::Absent, Presence::Absent) => Result::Ok(Ordering::Equal),
            (Presence::Present, Presence::Present) => context.compare(self.r#type, r1, r2),
            (Presence::Absent, Presence::Present) => {
                // "absent" < "present"
                Result::Ok(Ordering::Less)
            }
            (Presence::Present, Presence::Absent) => Result::Ok(Ordering::Greater),
        }
    }

    fn reference(&self, index: usize) -> Option<TypeRef> {
        if index == 0 {
            Some(self.r#type)
        } else {
            None
        }
    }
}

impl WithMetadata for TOption<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TOption<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TOption<'_> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder,
    {
        let field_type = builder.add(TReference::default().with_meta(NameDescription {
            name: "present_type",
            doc: "Type of the present value in an option.",
        }));

        TStruct::default()
            .add(Field::new(
                Identifier::try_from("type").unwrap(),
                field_type,
            ))
            .with_meta(NameDescription {
                name: "option",
                doc: "Can have a value (some; present) or no value (none; empty; absent).",
            })
    }
}
