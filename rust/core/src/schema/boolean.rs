use crate::common::error::LqError;
use crate::schema::core::{Context, SchemaBuilder};
use crate::schema::core::Type;
use crate::serialization::core::DeSerializer;
use crate::serialization::boolean::Bool;
use std::cmp::Ordering;
use crate::schema::doc_type::DocType;
use crate::schema::structure::TStruct;

#[derive(new, Clone, Debug)]
pub struct TBool;

impl Type for TBool {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        Bool::de_serialize(context.reader())?;
        Ok(())
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
        let bool1 = Bool::de_serialize(r1)?;
        let bool2 = Bool::de_serialize(r2)?;
        Result::Ok(bool1.cmp(&bool2))
    }

    fn build_schema<B>(_: &mut B) -> DocType<'static, TStruct>
        where
            B: SchemaBuilder,
    {
        // just an empty struct (but more fields will be added by the system)
        DocType::from(TStruct::builder()
            .build())
    }
}
