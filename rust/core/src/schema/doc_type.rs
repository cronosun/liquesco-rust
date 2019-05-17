use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Doc;
use crate::schema::core::SchemaBuilder;
use crate::schema::core::Type;
use crate::schema::structure::TStruct;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::Deref;

/// Wraps a type and adds an optional documentation to that type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocType<'doc, T: Type> {
    #[serde(flatten)]
    doc: Doc<'doc>,
    #[serde(flatten)]
    r#type: T,
}

impl<'doc, T: Type> From<T> for DocType<'doc, T> {
    fn from(r#type: T) -> Self {
        Self {
            r#type,
            doc: Doc::empty(),
        }
    }
}

impl<'doc, T: Type> Deref for DocType<'doc, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.r#type
    }
}

impl<'doc, T: Type> Type for DocType<'doc, T> {
    fn doc(&self) -> &Doc {
        &self.doc
    }

    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        self.r#type.validate(context)
    }

    fn compare<'c, C>(
        &self,
        context: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>,
    {
        self.r#type.compare(context, r1, r2)
    }

    fn build_schema<B>(builder: &mut B) -> TStruct
    where
        B: SchemaBuilder,
    {
        // TODO: Add fields for the doc...
        T::build_schema(builder)
    }
}
