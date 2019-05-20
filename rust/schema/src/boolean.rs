use liquesco_common::error::LqError;
use crate::core::Context;
use crate::core::Type;
use crate::doc_type::DocType;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::structure::TStruct;
use liquesco_core::serialization::boolean::Bool;
use liquesco_core::serialization::core::DeSerializer;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(new, Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
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
}

impl BaseTypeSchemaBuilder for TBool {
    fn build_schema<B>(_: &mut B) -> DocType<'static, TStruct<'static>>
    where
        B: SchemaBuilder,
    {
        // just an empty struct (but more fields will be added by the system)
        DocType::from(TStruct::default())
    }
}
