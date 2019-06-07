use crate::context::CmpContext;
use crate::context::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::WithMetadata;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::structure::TStruct;
use core::cmp::Ordering;
use liquesco_common::error::LqError;
use liquesco_serialization::binary::Binary;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::uuid::Uuid;
use serde::{Deserialize, Serialize};

/// A 16 byte Uuid (no other validation besides the length of 16 bytes is performed).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TUuid<'a> {
    meta: Meta<'a>,
}

impl<'a> Default for TUuid<'a> {
    fn default() -> Self {
        Self {
            meta: Meta::empty(),
        }
    }
}

impl Type for TUuid<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        // it's just a normal binary
        Uuid::de_serialize(context.reader())?;
        Result::Ok(())
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
        // compare like "normal" binaries
        let bin1 = Binary::de_serialize(r1)?;
        let bin2 = Binary::de_serialize(r2)?;
        Result::Ok(bin1.cmp(&bin2))
    }

    fn reference(&self, _: usize) -> Option<&TypeRef> {
        None
    }

    fn set_reference(&mut self, _: usize, _: TypeRef) -> Result<(), LqError> {
        LqError::err_new("This type has no references")
    }
}

impl WithMetadata for TUuid<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TUuid<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TUuid<'_> {
    fn build_schema<B>(_: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder<'static>,
    {
        // just an empty struct (but more fields will be added by the system)
        TStruct::default().with_doc("16 byte UUID; RFC 4122.")
    }
}
