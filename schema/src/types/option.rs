use crate::context::CmpContext;
use crate::context::ValidationContext;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::WithMetadata;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::types::key_ref::TKeyRef;
use crate::types::structure::Field;
use crate::types::structure::TStruct;
use liquesco_common::error::LqError;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::types::option::Presence;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

/// Data of the option type have two variants: Absent or present and a value.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TOption<'a> {
    meta: Meta<'a>,
    r#type: TypeRef,
}

impl<'a> TOption<'a> {
    /// The type of the present value.
    pub fn r#type(&self) -> &TypeRef {
        &self.r#type
    }

    /// Creates a new option type.
    pub fn new(r#type: TypeRef) -> Self {
        Self {
            meta: Meta::empty(),
            r#type,
        }
    }
}

impl Type for TOption<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: ValidationContext<'c>,
    {
        let presence = Presence::de_serialize(context.reader())?;

        match presence {
            Presence::Absent => Result::Ok(()),
            Presence::Present => context.validate(&self.r#type),
        }
    }

    fn compare<'c, C>(
        &self,
        context: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: CmpContext<'c>,
    {
        let presence1 = Presence::de_serialize(r1)?;
        let presence2 = Presence::de_serialize(r2)?;

        match (presence1, presence2) {
            (Presence::Absent, Presence::Absent) => Result::Ok(Ordering::Equal),
            (Presence::Present, Presence::Present) => context.compare(&self.r#type, r1, r2),
            (Presence::Absent, Presence::Present) => {
                // "absent" < "present"
                Result::Ok(Ordering::Less)
            }
            (Presence::Present, Presence::Absent) => Result::Ok(Ordering::Greater),
        }
    }

    fn reference(&self, index: usize) -> Option<&TypeRef> {
        if index == 0 {
            Some(&self.r#type)
        } else {
            None
        }
    }

    fn set_reference(&mut self, index: usize, type_ref: TypeRef) -> Result<(), LqError> {
        if index == 0 {
            self.r#type = type_ref;
            Ok(())
        } else {
            LqError::err_new(format!("Option has no type at index {}", index))
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
        B: SchemaBuilder<'static>,
    {
        let field_type = builder.add_unwrap(
            "present_type",
            TKeyRef::default().with_doc("Type of the present value in an option."),
        );

        TStruct::default()
            .add(Field::new(
                Identifier::try_from("type").unwrap(),
                field_type,
            ))
            .with_doc("Can have a value (some; present) or no value (none; empty; absent).")
    }
}
