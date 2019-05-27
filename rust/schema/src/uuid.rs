use crate::core::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::structure::TStruct;
use core::cmp::Ordering;
use liquesco_common::error::LqError;
use liquesco_serialization::binary::Binary;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::metadata::WithMetadata;
use crate::metadata::MetadataSetter;
use crate::metadata::Meta;
use crate::metadata::NameDescription;

/// Note: We cannot have a unit struct (since serde complains when flattening is enabled)
#[derive(new, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TUuid<'a> {
    #[new(value = "Meta::empty()")]
    pub meta : Meta<'a>,
}

impl<'a> Default for TUuid<'a> {
    fn default() -> Self {
        Self {
            meta : Meta::empty(),
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
        C: Context<'c>,
    {
        // compare like "normal" binaries
        let bin1 = Binary::de_serialize(r1)?;
        let bin2 = Binary::de_serialize(r2)?;
        Result::Ok(bin1.cmp(&bin2))
    }

    fn reference(&self, _: usize) -> Option<TypeRef> {
        None
    }
}

impl WithMetadata for TUuid<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TUuid<'a> {
    fn set_meta(&mut self, meta : Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TUuid<'_> {
    fn build_schema<B>(_: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder,
    {
        // just an empty struct (but more fields will be added by the system)
        TStruct::default().with_meta(NameDescription {
            name : "uuid",
            description : "16 byte UUID; RFC 4122."
        })
    }
}
