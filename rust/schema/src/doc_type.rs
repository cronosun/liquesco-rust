use crate::core::TypeRef;
use crate::core::Doc;
use crate::core::Type;
use crate::core::{
    Context, DOC_MAX_LEN_UTF8_BYTES, DOC_MIN_LEN_UTF8_BYTES, MAX_IMPLEMENTS_ELEMENTS,
    MIN_IMPLEMENTS_ELEMENTS,
};
use crate::identifier::Identifier;
use crate::option::TOption;
use crate::schema_builder::BuildsOwnSchema;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::seq;
use crate::seq::TSeq;
use crate::structure::Field;
use crate::structure::TStruct;
use crate::unicode::{LengthType, TUnicode};
use crate::uuid::TUuid;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::U32IneRange;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

/// Wraps a type and adds an optional documentation to that type.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
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

    fn reference(&self, index : usize) -> Option<TypeRef> {
         self.r#type.reference(index) 
    }
}

impl<'doc, T: Type> DocType<'doc, T> {
    pub fn with_name<ID: Into<Identifier<'doc>>>(mut self, name: ID) -> Self {
        self.doc.set_name(name);
        self
    }

    pub fn with_name_unwrap<TryErr: Debug, ID: TryInto<Identifier<'doc>, Error = TryErr>>(
        self,
        name: ID,
    ) -> Self {
        self.with_name(name.try_into().unwrap())
    }

    pub fn with_description<D: Into<Cow<'doc, str>>>(mut self, description: D) -> Self {
        self.doc.set_description(description);
        self
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
        let field_name = builder.add(
            DocType::from(TOption::new(identifier_ref))
                .with_name_unwrap("maybe_name")
                .with_description("An optional name for the type."),
        );
        let description_ref = builder.add(
            DocType::from(
                TUnicode::try_new(
                    DOC_MIN_LEN_UTF8_BYTES as u64,
                    DOC_MAX_LEN_UTF8_BYTES as u64,
                    LengthType::Utf8Byte,
                )
                .unwrap(),
            )
            .with_name_unwrap("description")
            .with_description(
                "Describes / documents the type. Use human readable text. \
                 No markup is allowed.",
            ),
        );
        let field_description = builder.add(
            DocType::from(TOption::new(description_ref))
                .with_name_unwrap("maybe_description")
                .with_description("Optional type description / documentation."),
        );
        let uuid_ref = builder.add(
            DocType::from(TUuid::default())
                .with_name_unwrap("implements_uuid")
                .with_description(
                    "UUID to describe the conformance / implementation / \
                     protocol of this type uniquely.",
                ),
        );
        let uuid_seq = builder.add(DocType::from(
            TSeq {
                element : uuid_ref,
                length : U32IneRange::try_new(
                    MIN_IMPLEMENTS_ELEMENTS as u32,
                    MAX_IMPLEMENTS_ELEMENTS as u32).unwrap(),
                ordering : seq::Ordering::Sorted {
                    direction : seq::Direction::Ascending,
                    unique: true
                },
                multiple_of : None,
            }
        )
        .with_name_unwrap("implements")
        .with_description(
            "A sequence of 'things' this type implements. What can this be used \
        for? Say for example your type is an ASCII type. With that information we can't \
        really say what ASCII is for. Say for example that ASCII has a special meaning in your \
        company: It's a part number. So you can give that ASCII type a UUID to declare 'this \
        type is a part number'. This makes it possible to find part numbers company wide."));
        let field_implements =
            builder.add(DocType::from(TOption::new(uuid_seq)).with_name_unwrap("maybe_implements"));

        let mut inner_struct = T::build_schema(builder);
        inner_struct.prepend(Field {
            name: Identifier::try_from("implements").unwrap(),
            r#type: field_implements,
        });
        inner_struct.prepend(Field {
            name: Identifier::try_from("description").unwrap(),
            r#type: field_description,
        });
        inner_struct.prepend(Field {
            name: Identifier::try_from("name").unwrap(),
            r#type: field_name,
        });

        inner_struct
    }
}
