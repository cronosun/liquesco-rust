use crate::context::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::NameDescription;
use crate::metadata::NameOnly;
use crate::metadata::WithMetadata;
use crate::range::{Inclusion, TRange};
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::structure::Field;
use crate::structure::TStruct;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::U64IneRange;
use liquesco_common::range::LqRangeBounds;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::uint::UInt64;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

/// A 64-bit unsigned integer.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TUInt<'a> {
    meta: Meta<'a>,
    range: U64IneRange,
}

impl<'a> TUInt<'a> {
    pub fn new(range: U64IneRange) -> Self {
        Self {
            meta: Meta::empty(),
            range,
        }
    }

    pub fn try_new(min: u64, max: u64) -> Result<Self, LqError> {
        Result::Ok(TUInt::new(U64IneRange::try_new(
            "Unsigned integer range",
            min,
            max,
        )?))
    }

    pub fn range(&self) -> &U64IneRange {
        &self.range
    }
}

impl Type for TUInt<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let int_value = UInt64::de_serialize(context.reader())?;
        self.range
            .require_within("Unsigned integer schema validation", &int_value)?;
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
        let int1 = UInt64::de_serialize(r1)?;
        let int2 = UInt64::de_serialize(r2)?;
        Result::Ok(int1.cmp(&int2))
    }

    fn reference(&self, _: usize) -> Option<TypeRef> {
        None
    }
}

impl WithMetadata for TUInt<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TUInt<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TUInt<'_> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder,
    {
        let element = builder.add(
            TUInt::try_new(std::u64::MIN, std::u64::MAX)
                .unwrap()
                .with_meta(NameOnly {
                    name: "uint_range_element",
                }),
        );

        let field_range = builder.add(
            TRange::new(element, Inclusion::BothInclusive, false).with_meta(NameDescription {
                name: "uint_range",
                doc: "The range within the integer must be. Both (start and end) \
                      are inclusive.",
            }),
        );

        TStruct::default()
            .add(Field::new(
                Identifier::try_from("range").unwrap(),
                field_range,
            ))
            .with_meta(NameDescription {
                name: "uint",
                doc: "Unsigned integer - maximum 64 bit.",
            })
    }
}
