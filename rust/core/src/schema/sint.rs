use crate::common::error::LqError;
use crate::common::ine_range::{I64IneRange, U32IneRange};
use crate::common::range::LqRangeBounds;
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
use crate::serialization::sint::SInt64;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

#[derive(new, Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
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
}

impl BaseTypeSchemaBuilder for TSInt {
    fn build_schema<B>(builder: &mut B) -> DocType<'static, TStruct<'static>>
    where
        B: SchemaBuilder,
    {
        let element = builder.add(DocType::from(
            TSInt::try_new(std::i64::MIN, std::i64::MAX).unwrap(),
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
