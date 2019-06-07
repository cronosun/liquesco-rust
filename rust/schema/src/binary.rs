use crate::context::CmpContext;
use crate::context::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::WithMetadata;
use crate::range::Inclusion;
use crate::range::TRange;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::structure::Field;
use crate::structure::TStruct;
use crate::uint::TUInt;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::U64IneRange;
use liquesco_common::range::LqRangeBounds;
use liquesco_serialization::binary::Binary;
use liquesco_serialization::core::DeSerializer;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

/// Arbitrary binary data.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TBinary<'a> {
    meta: Meta<'a>,
    length: U64IneRange,
}

impl<'a> TBinary<'a> {
    pub fn new(length: U64IneRange) -> Self {
        Self {
            meta: Default::default(),
            length,
        }
    }

    pub fn try_new(min_length: u64, max_length: u64) -> Result<Self, LqError> {
        Result::Ok(Self {
            meta: Meta::empty(),
            length: U64IneRange::try_new("Binary length range", min_length, max_length)?,
        })
    }

    /// The length in bytes.
    pub fn length(&self) -> &U64IneRange {
        &self.length
    }
}

impl Type for TBinary<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let bytes = Binary::de_serialize(context.reader())?;
        let length = bytes.len();
        let length_u64 = u64::try_from(length)?;
        self.length
            .require_within("Binary length validation", &length_u64)?;
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
        let bytes1 = Binary::de_serialize(r1)?;
        let bytes2 = Binary::de_serialize(r2)?;
        // lex compare
        Result::Ok(bytes1.cmp(&bytes2))
    }

    fn reference(&self, _: usize) -> Option<&TypeRef> {
        None
    }

    fn set_reference(&mut self, _: usize, _: TypeRef) -> Result<(), LqError> {
        LqError::err_new("This type has no references")
    }
}

impl WithMetadata for TBinary<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TBinary<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TBinary<'_> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder<'static>,
    {
        let range_element = builder.add_unwrap(
            "binary_length_element",
            TUInt::try_new(std::u64::MIN, std::u64::MAX).unwrap(),
        );
        let field_length = builder.add_unwrap(
            "binary_length",
            TRange::new(range_element, Inclusion::BothInclusive, false),
        );

        TStruct::default()
            .add(Field::new(
                Identifier::try_from("length").unwrap(),
                field_length,
            ))
            .with_doc("Arbitrary binary.")
    }
}
