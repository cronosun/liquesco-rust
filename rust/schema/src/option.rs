use liquesco_common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Type;
use crate::schema::core::TypeRef;
use crate::schema::doc_type::DocType;
use crate::schema::identifier::Identifier;
use crate::schema::reference::TReference;
use crate::schema::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::schema::structure::Field;
use crate::schema::structure::TStruct;
use crate::serialization::core::DeSerializer;
use crate::serialization::option::Presence;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

#[derive(new, Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct TOption {
    pub r#type: TypeRef,
}

impl Type for TOption {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let presence = Presence::de_serialize(context.reader())?;

        match presence {
            Presence::Absent => Result::Ok(()),
            Presence::Present => context.validate(self.r#type),
        }
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
        let presence1 = Presence::de_serialize(r1)?;
        let presence2 = Presence::de_serialize(r2)?;

        match (presence1, presence2) {
            (Presence::Absent, Presence::Absent) => Result::Ok(Ordering::Equal),
            (Presence::Present, Presence::Present) => context.compare(self.r#type, r1, r2),
            (Presence::Absent, Presence::Present) => {
                // "absent" < "present"
                Result::Ok(Ordering::Less)
            }
            (Presence::Present, Presence::Absent) => Result::Ok(Ordering::Greater),
        }
    }
}

impl BaseTypeSchemaBuilder for TOption {
    fn build_schema<B>(builder: &mut B) -> DocType<'static, TStruct<'static>>
    where
        B: SchemaBuilder,
    {
        let field_type = builder.add(DocType::from(TReference));

        DocType::from(TStruct::default().add(Field::new(
            Identifier::try_from("type").unwrap(),
            field_type,
        )))
    }
}
