use crate::identifier::Identifier;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::U32IneRange;
use liquesco_serialization::uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::convert::TryFrom;

use crate::option::TOption;
use crate::schema_builder::BuildsOwnSchema;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::seq;
use crate::seq::{Sorted, TSeq};
use crate::structure::Field;
use crate::structure::TStruct;
use crate::unicode::{LengthType, TUnicode};
use crate::uuid::TUuid;
use serde::export::PhantomData;

/// Something that has metadata.
pub trait WithMetadata {
    /// Metadata for that type.
    fn meta(&self) -> &Meta;
}

/// Something that supports mutating metadata.
pub trait MetadataSetter<'m>: WithMetadata {
    fn set_meta(&mut self, meta: Meta<'m>);

    fn with_meta<M: Into<Meta<'m>>>(mut self, meta: M) -> Self
    where
        Self: Sized,
    {
        self.set_meta(meta.into());
        self
    }

    fn with_doc<D : Into<Cow<'m, str>>>(mut self, doc : D) -> Self
    where Self : Sized {
        self.set_meta(Meta {
            doc:    Some(doc.into()),
            implements : self.meta().implements.clone()
        });
        self
    }
}

/// Minimum length (utf-8 bytes) the documentation of a type must have.
pub const DOC_MIN_LEN_UTF8_BYTES: usize = 1;
/// Maximum length (utf-8 bytes) the documentation of a type can have.
pub const DOC_MAX_LEN_UTF8_BYTES: usize = 4000;

/// The metadata of a type.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Meta<'a> {
    pub doc: Option<Cow<'a, str>>,
    pub implements: Option<Implements>,
}

impl<'a> Meta<'a> {
    pub const fn empty() -> Self {
        Self {
            doc: None,
            implements: None,
        }
    }

    pub fn doc(&self) -> Option<&str> {
        if let Some(desc) = &self.doc {
            Some(desc)
        } else {
            None
        }
    }

    pub fn implements(&self) -> &[Uuid] {
        if let Some(implements) = &self.implements {
            &implements.0
        } else {
            &[]
        }
    }

    pub fn set_doc<D>(&mut self, doc: D)
    where
        D: Into<Cow<'a, str>>,
    {
        self.doc = Some(doc.into());
    }

    pub fn add_implements<U>(&mut self, uuid: U) -> Result<(), LqError>
    where
        U: Into<Uuid>,
    {
        if let None = self.implements {
            self.implements = Option::Some(Implements::try_new(&[uuid.into()])?);
        } else {
            let mut implements = self.implements.take().unwrap();
            implements.add(uuid.into())?;
            self.implements = Some(implements);
        }
        Ok(())
    }
}

impl<'a> Default for Meta<'a> {
    fn default() -> Self {
        Meta::empty()
    }
}

pub const MIN_IMPLEMENTS_ELEMENTS: usize = 1;
pub const MAX_IMPLEMENTS_ELEMENTS: usize = 255;

/// What a type implements.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Implements(Vec<Uuid>);

impl Implements {
    pub fn try_new(implements: &[Uuid]) -> Result<Self, LqError> {
        let number = implements.len();
        if number < MIN_IMPLEMENTS_ELEMENTS {
            LqError::err_new("You need at least one element in 'implements'.")
        } else if number > MAX_IMPLEMENTS_ELEMENTS {
            LqError::err_new(format!(
                "There are too many implements elements. Maximum is {:?}; got {:?} elements.",
                MAX_IMPLEMENTS_ELEMENTS, number
            ))
        } else {
            Ok(Implements(Vec::from(implements)))
        }
    }

    pub fn add(&mut self, implements: Uuid) -> Result<(), LqError> {
        let number = self.0.len() + 1;
        if number > MAX_IMPLEMENTS_ELEMENTS {
            LqError::err_new(format!(
                "There are too many implements elements. Maximum is {:?}; got {:?} elements.",
                MAX_IMPLEMENTS_ELEMENTS, number
            ))
        } else {
            self.0.push(implements);
            Ok(())
        }
    }
}

pub(crate) struct WithMetaSchemaBuilder<T: BaseTypeSchemaBuilder> {
    _phantom: PhantomData<T>,
}

impl<T: BaseTypeSchemaBuilder> BaseTypeSchemaBuilder for WithMetaSchemaBuilder<T> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder<'static>,
    {
        // Adding fields for the doc
        let doc_ref = builder.add_unwrap(
            "documentation",
            TUnicode::try_new(
                DOC_MIN_LEN_UTF8_BYTES as u64,
                DOC_MAX_LEN_UTF8_BYTES as u64,
                LengthType::Utf8Byte,
            )
            .unwrap()
            .with_doc("Describes / documents the type. Use human readable text. \
                      No markup is allowed."),
        );
        let field_doc = builder.add_unwrap(
            "maybe_doc",
            TOption::new(doc_ref).with_doc( "Optional type description / documentation."));
        let uuid_ref = builder.add_unwrap(
            "implements_uuid",
            TUuid::default().with_doc( "UUID to describe the conformance / implementation / \
                  protocol of this type uniquely."));
        let uuid_seq = builder.add_unwrap(
            "implements",
            TSeq::new(
                uuid_ref,
                U32IneRange::try_new("Doc type implements",
                                     MIN_IMPLEMENTS_ELEMENTS as u32,
                                     MAX_IMPLEMENTS_ELEMENTS as u32).unwrap())
                .with_sorted(Sorted {
                    direction: seq::Direction::Ascending,
                    unique: true,
                })
                .with_doc("A sequence of 'things' this type implements. What can this be used \
        for? Say for example your type is an ASCII type. With that information we can't \
        really say what ASCII is for. Say for example that ASCII has a special meaning in your \
        company: It's a part number. So you can give that ASCII type a UUID to declare 'this \
        type is a part number'. This makes it possible to find part numbers company wide."));

        let field_implements = builder.add_unwrap(
            "maybe_implements",
            TOption::new(uuid_seq));

        let meta_struct = TStruct::default()
            .add(Field::new(
                Identifier::try_from("doc").unwrap(),
                 field_doc,
        ))
            .add(Field::new(
                Identifier::try_from("implements").unwrap(),
                field_implements,
            )).with_doc( "Meta information about the type. You can optionally specify a name, a description/\
        documentation and information about implementation/conformance.");
        let meta_ref = builder.add_unwrap(
            "meta",
            meta_struct);

        let mut inner_struct = T::build_schema(builder);
        inner_struct.prepend(Field::new(Identifier::try_from("meta").unwrap(), meta_ref));

        inner_struct
    }
}
