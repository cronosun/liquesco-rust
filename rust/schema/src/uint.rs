use liquesco_common::error::LqError;
use liquesco_common::ine_range::{U32IneRange, U64IneRange};
use liquesco_common::range::LqRangeBounds;
use crate::schema::core::Context;
use crate::schema::core::Type;
use crate::schema::doc_type::DocType;
use crate::schema::identifier::Identifier;
use crate::schema::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::schema::seq::Ordering as SeqOrdering;
use crate::schema::seq::{Direction, TSeq};
use crate::schema::structure::Field;
use crate::schema::structure::TStruct;
use crate::serialization::core::DeSerializer;
use crate::serialization::uint::UInt64;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

#[derive(new, Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct TUInt {
    pub range: U64IneRange,
}

impl TUInt {
    pub fn try_new(min: u64, max: u64) -> Result<Self, LqError> {
        Result::Ok(TUInt::new(U64IneRange::try_new_msg(
            "Unsigned integer range",
            min,
            max,
        )?))
    }
}

impl Type for TUInt {
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
}

impl BaseTypeSchemaBuilder for TUInt {
    fn build_schema<B>(builder: &mut B) -> DocType<'static, TStruct<'static>>
    where
        B: SchemaBuilder,
    {
        let element = builder.add(DocType::from(
            TUInt::try_new(std::u64::MIN, std::u64::MAX).unwrap(),
        ));
        let field_range = builder.add(DocType::from(TSeq {
            element,
            length: U32IneRange::try_new(2, 2).unwrap(),
            ordering: SeqOrdering::Sorted {
                direction: Direction::Ascending,
                unique: true,
            },
            multiple_of: None,
        }));

        DocType::from(TStruct::default().add(Field::new(
            Identifier::try_from("range").unwrap(),
            field_range,
        )))
    }
}
