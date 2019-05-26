use crate::core::TypeRef;
use crate::core::Context;
use crate::core::Type;
use crate::doc_type::DocType;
use crate::identifier::Identifier;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::seq::Ordering as SeqOrdering;
use crate::seq::{Direction, TSeq};
use crate::structure::Field;
use crate::structure::TStruct;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::{I64IneRange, U32IneRange};
use liquesco_common::range::LqRangeBounds;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::sint::SInt64;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

#[derive(new, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TSInt {
    pub range: I64IneRange,
}

impl TSInt {
    pub fn try_new(min: i64, max: i64) -> Result<Self, LqError> {
        Result::Ok(TSInt::new(I64IneRange::try_new_msg(
            "Signed integer range",
            min,
            max,
        )?))
    }

    pub fn range(&self) -> &I64IneRange {
        &self.range
    }
}

impl Type for TSInt {
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

impl BaseTypeSchemaBuilder for TSInt {
    fn build_schema<B>(builder: &mut B) -> DocType<'static, TStruct<'static>>
    where
        B: SchemaBuilder,
    {
        let element = builder.add(
            DocType::from(TSInt::try_new(std::i64::MIN, std::i64::MAX).unwrap())
                .with_name_unwrap("sint_range_element"),
        );
        let field_range = builder.add(
            DocType::from(TSeq {
                element,
                length: U32IneRange::try_new(2, 2).unwrap(),
                ordering: SeqOrdering::Sorted {
                    direction: Direction::Ascending,
                    unique: true,
                },
                multiple_of: None,
            })
            .with_name_unwrap("sint_range")
            .with_description(
                "The range within the integer must be. Both (start and end) \
                 are inclusive.",
            ),
        );

        DocType::from(TStruct::default().add(Field::new(
            Identifier::try_from("range").unwrap(),
            field_range,
        )))
        .with_name_unwrap("sint")
        .with_description("Signed integer - maximum 64 bit.")
    }
}
