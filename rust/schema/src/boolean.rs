use crate::context::CmpContext;
use crate::context::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::WithMetadata;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::structure::TStruct;
use liquesco_common::error::LqError;
use liquesco_serialization::boolean::Bool;
use liquesco_serialization::core::DeSerializer;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// The boolean type (true / false).
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TBool<'a> {
    meta: Meta<'a>,
}

impl<'a> Default for TBool<'a> {
    fn default() -> Self {
        Self {
            meta: Meta::empty(),
        }
    }
}

impl Type for TBool<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        Bool::de_serialize(context.reader())?;
        Ok(())
    }

    fn compare<'c, C>(
        &self,
        _: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: CmpContext<'c>,
    {
        let bool1 = Bool::de_serialize(r1)?;
        let bool2 = Bool::de_serialize(r2)?;
        Result::Ok(bool1.cmp(&bool2))
    }

    fn reference(&self, _: usize) -> Option<&TypeRef> {
        None
    }

    fn set_reference(&mut self, _: usize, _: TypeRef) -> Result<(), LqError> {
        LqError::err_new("This type has no references")
    }
}

impl WithMetadata for TBool<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TBool<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TBool<'_> {
    fn build_schema<B>(_: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder<'static>,
    {
        // just an empty struct (but more fields will be added by the system)
        TStruct::default().with_doc("A boolean: Can either be true or false.")
    }
}
