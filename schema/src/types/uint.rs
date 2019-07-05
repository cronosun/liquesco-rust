use crate::context::CmpContext;
use crate::context::ValidationContext;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::WithMetadata;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::types::range::{Inclusion, TRange};
use crate::types::structure::Field;
use crate::types::structure::TStruct;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::U128IneRange;
use liquesco_common::range::LqRangeBounds;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::types::uint::UInt128;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use crate::types::tint::TInt;
use liquesco_common::int_memory::IntMemory;

/// A 128-bit unsigned integer.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TUInt<'a> {
    meta: Meta<'a>,
    range: U128IneRange,
}

impl<'a> TUInt<'a> {
    pub fn new(range: U128IneRange) -> Self {
        Self {
            meta: Meta::empty(),
            range,
        }
    }

    pub fn try_new<TMin, TMax>(min: TMin, max: TMax) -> Result<Self, LqError>
    where
        TMin: Into<u128>,
        TMax: Into<u128>,
    {
        Result::Ok(TUInt::new(U128IneRange::try_new(
            "Unsigned integer range",
            min.into(),
            max.into(),
        )?))
    }
}

impl<'a> TInt<u128> for TUInt<'a> {
    fn range(&self) -> &U128IneRange {
        &self.range
    }

    fn memory(&self) -> IntMemory {
        let start = IntMemory::from(self.range().start());
        let end = IntMemory::from(self.range().end());
        start.max(end)
    }
}

impl Type for TUInt<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: ValidationContext<'c>,
    {
        let int_value = UInt128::de_serialize(context.reader())?;
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
        C: CmpContext<'c>,
    {
        let int1 = UInt128::de_serialize(r1)?;
        let int2 = UInt128::de_serialize(r2)?;
        Result::Ok(int1.cmp(&int2))
    }

    fn reference(&self, _: usize) -> Option<&TypeRef> {
        None
    }

    fn set_reference(&mut self, _: usize, _: TypeRef) -> Result<(), LqError> {
        LqError::err_new("This type has no references")
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
        B: SchemaBuilder<'static>,
    {
        let element = builder.add_unwrap(
            "uint_range_element",
            TUInt::try_new(std::u128::MIN, std::u128::MAX).unwrap(),
        );

        let field_range = builder.add_unwrap(
            "uint_range",
            TRange::new(element, Inclusion::BothInclusive, false).with_doc(
                "The range within the integer must be. Both (start and end) \
                 are inclusive.",
            ),
        );

        TStruct::default()
            .add(Field::new(
                Identifier::try_from("range").unwrap(),
                field_range,
            ))
            .with_doc("Unsigned integer - maximum 128 bit.")
    }
}
