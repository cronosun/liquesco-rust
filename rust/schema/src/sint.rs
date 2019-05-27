use crate::core::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::range::{Inclusion, TRange};
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::structure::Field;
use crate::structure::TStruct;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::I64IneRange;
use liquesco_common::range::LqRangeBounds;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::sint::SInt64;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use crate::metadata::WithMetadata;
use crate::metadata::MetadataSetter;
use crate::metadata::Meta;
use crate::metadata::NameOnly;
use crate::metadata::NameDescription;

#[derive(new, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TSInt<'a> {
    #[new(value = "Meta::empty()")]
    pub meta : Meta<'a>,
    pub range: I64IneRange,
}

impl<'a> TSInt<'a> {
    pub fn try_new(min: i64, max: i64) -> Result<Self, LqError> {
        Result::Ok(TSInt::new(I64IneRange::try_new(
            "Signed integer range",
            min,
            max,
        )?))
    }

    pub fn range(&self) -> &I64IneRange {
        &self.range
    }
}

impl Type for TSInt<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let int_value = SInt64::de_serialize(context.reader())?;
        self.range
            .require_within("Signed integer schema validation", &int_value)?;
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
        let int1 = SInt64::de_serialize(r1)?;
        let int2 = SInt64::de_serialize(r2)?;
        Result::Ok(int1.cmp(&int2))
    }

    fn reference(&self, _: usize) -> Option<TypeRef> {
        None
    }
}

impl WithMetadata for TSInt<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TSInt<'a> {
    fn set_meta(&mut self, meta : Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TSInt<'_> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder,
    {
        let element = builder.add(
            TSInt::try_new(std::i64::MIN, std::i64::MAX).unwrap()
                .with_meta(NameOnly {
                    name : "sint_range_element"
                })
        );

        let field_range = builder.add(
            TRange {
                meta : Meta::empty(),
                element,
                inclusion: Inclusion::BothInclusive,
                allow_empty: false,
            }.with_meta(NameDescription {
                name: "sint_range",
                description:  "The range within the integer must be. Both (start and end) \
                 are inclusive."
            })
        );

        TStruct::default().add(Field::new(
            Identifier::try_from("range").unwrap(),
            field_range,
        )).with_meta(NameDescription {
            name : "sint",
            description : "Signed integer - maximum 64 bit."
        })
    }
}
