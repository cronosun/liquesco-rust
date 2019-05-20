use crate::common::error::LqError;
use crate::schema::core::Doc;
use crate::schema::core::Type;
use crate::schema::core::{
    Context, DOC_MAX_LEN_UTF8_BYTES, DOC_MIN_LEN_UTF8_BYTES, MAX_IMPLEMENTS_ELEMENTS,
    MIN_IMPLEMENTS_ELEMENTS,
};
use crate::schema::identifier::Identifier;
use crate::schema::option::TOption;
use crate::schema::schema_builder::BuildsOwnSchema;
use crate::schema::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::schema::seq::TSeq;
use crate::schema::structure::Field;
use crate::schema::structure::TStruct;
use crate::schema::unicode::{LengthType, TUnicode};
use crate::schema::uuid::TUuid;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};

/// Wraps a type and adds an optional documentation to that type.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq)]
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

impl<'doc, T: Type> DerefMut for DocType<'doc, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.r#type
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
}

impl<T> BaseTypeSchemaBuilder for DocType<'_, T>
where
    T: BaseTypeSchemaBuilder + Type,
{
    fn build_schema<B>(builder: &mut B) -> DocType<'static, TStruct<'static>>
    where
        B: SchemaBuilder,
    {
        // Adding fields for the doc
        let identifier_ref = Identifier::build_schema(builder);
        let field_name = builder.add(DocType::from(TOption::new(identifier_ref)));
        let description_ref = builder.add(DocType::from(
            TUnicode::try_new(
                DOC_MIN_LEN_UTF8_BYTES as u64,
                DOC_MAX_LEN_UTF8_BYTES as u64,
                LengthType::Utf8Byte,
            )
            .unwrap(),
        ));
        let field_description = builder.add(DocType::from(TOption::new(description_ref)));
        let uuid_ref = builder.add(DocType::from(TUuid));
        let uuid_seq = builder.add(DocType::from(
            TSeq::try_new(
                uuid_ref,
                MIN_IMPLEMENTS_ELEMENTS as u32,
                MAX_IMPLEMENTS_ELEMENTS as u32,
            )
            .unwrap(),
        ));
        let field_implements = builder.add(DocType::from(TOption::new(uuid_seq)));

        let mut inner_struct = T::build_schema(builder);
        inner_struct.prepend(Field {
            identifier: Identifier::try_from("name").unwrap(),
            r#type: field_name,
        });
        inner_struct.prepend(Field {
            identifier: Identifier::try_from("description").unwrap(),
            r#type: field_description,
        });
        inner_struct.prepend(Field {
            identifier: Identifier::try_from("implements").unwrap(),
            r#type: field_implements,
        });

        inner_struct
    }
}
