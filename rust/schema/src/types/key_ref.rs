use crate::context::CmpContext;
use crate::context::ValidationContext;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::WithMetadata;
use crate::schema_builder::BaseTypeSchemaBuilder;
use crate::schema_builder::SchemaBuilder;
use crate::types::structure::Field;
use crate::types::structure::TStruct;
use crate::types::uint::TUInt;
use liquesco_common::error::LqError;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::types::uint::UInt32;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

/// References a key in the nth outer map (see level).
///
/// Technical detail: It's just a number. That number is the index in the outer map. So it's
/// always >=0 and < number of items in the map. It can only be used when there's an outer
/// map in the schema. When there's no outer map, schema is valid but data validation will
/// always fail.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TKeyRef<'a> {
    meta: Meta<'a>,
    level: u32,
}

impl<'a> Default for TKeyRef<'a> {
    fn default() -> Self {
        Self {
            meta: Meta::empty(),
            level: 0,
        }
    }
}

impl<'a> TKeyRef<'a> {
    /// The level determines which outer map is references. Usually the level is 0, in this case
    /// it's the next outer map.
    pub fn with_level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }

    pub fn level(&self) -> u32 {
        self.level
    }
}

impl Type for TKeyRef<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: ValidationContext<'c>,
    {
        let ref_int = UInt32::de_serialize(context.reader())?;
        if let Some(ref_info) = context.key_ref_info(self.level) {
            if ref_int >= ref_info.map_len() {
                LqError::err_new(format!(
                    "You're referencing key at index {} in a map but \
                     the map only has {} keys.",
                    ref_int,
                    ref_info.map_len()
                ))
            } else {
                Ok(())
            }
        } else {
            LqError::err_new(format!(
                "You're trying to reference key {} in a map but \
                 there's no map that's currently being processed; or there's no map at level {}. \
                 Key references can only \
                 be within a map.",
                ref_int, self.level
            ))
        }
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
        let int1 = UInt32::de_serialize(r1)?;
        let int2 = UInt32::de_serialize(r2)?;
        Result::Ok(int1.cmp(&int2))
    }

    fn reference(&self, _: usize) -> Option<&TypeRef> {
        None
    }

    fn set_reference(&mut self, _: usize, _: TypeRef) -> Result<(), LqError> {
        LqError::err_new("This type has no references")
    }
}

impl WithMetadata for TKeyRef<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TKeyRef<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TKeyRef<'_> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder<'static>,
    {
        let level_type = builder.add_unwrap(
            "level",
            TUInt::try_new(0u32, u64::from(std::u32::MAX))
                .unwrap()
                .with_doc(
                    "Specifies which outer map you want to reference. This is usually \
                     0: In this case you reference keys from the next outer map. Note: Those map \
                     that do not provide anchors that can be referenced are ignored.",
                ),
        );
        TStruct::default()
            .add(Field::new(
                Identifier::try_from("level").unwrap(),
                level_type,
            ))
            .with_doc(
                "Key references can reference keys from outer types that supports references \
                 (provide anchors that can be referenced): Maps and RootMaps.",
            )
    }
}
