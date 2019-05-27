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
use crate::seq::TSeq;
use crate::structure::Field;
use crate::structure::TStruct;
use crate::unicode::{LengthType, TUnicode};
use crate::uuid::TUuid;
use serde::export::PhantomData;

pub trait WithMetadata {
    /// Metadata for that type.
    fn meta(&self) -> &Meta {
        // types usually have no documentation. We use a special wrapper that adds
        // documentation to a type. See `DocType`.
        EMPTY_META
    }
}

pub trait MetadataSetter<'m>: WithMetadata {
    fn set_meta(&mut self, meta: Meta<'m>);

    fn with_meta<M: Into<Meta<'m>>>(mut self, meta: M) -> Self
    where
        Self: Sized,
    {
        self.set_meta(meta.into());
        self
    }
}

pub const DOC_MIN_LEN_UTF8_BYTES: usize = 1;
pub const DOC_MAX_LEN_UTF8_BYTES: usize = 4000;

/// The metadata of a type.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Meta<'a> {
    pub name: Option<Identifier<'a>>,
    pub description: Option<Cow<'a, str>>,
    pub implements: Option<Implements>,
}

pub struct NameDescription<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

const EMPTY_META: &Meta = &Meta::empty();

impl<'a> Into<Meta<'a>> for NameDescription<'a> {
    fn into(self) -> Meta<'a> {
        Meta {
            name: Some(Identifier::try_from(self.name).unwrap()),
            description: Some(Cow::Borrowed(self.description)),
            implements: None,
        }
    }
}

pub struct NameOnly<'a> {
    pub name: &'a str,
}

impl<'a> Into<Meta<'a>> for NameOnly<'a> {
    fn into(self) -> Meta<'a> {
        Meta {
            name: Some(Identifier::try_from(self.name).unwrap()),
            description: None,
            implements: None,
        }
    }
}

impl<'a> Meta<'a> {
    pub const fn empty() -> Self {
        Self {
            name: None,
            description: None,
            implements: None,
        }
    }

    pub fn name(&self) -> Option<&Identifier<'a>> {
        if let Some(identifier) = &self.name {
            Some(&identifier)
        } else {
            None
        }
    }

    pub fn description(&self) -> Option<&str> {
        if let Some(desc) = &self.description {
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

    pub fn set_name<I>(&mut self, name: I)
    where
        I: Into<Identifier<'a>>,
    {
        self.name = Some(name.into());
    }

    pub fn set_description<D>(&mut self, description: D)
    where
        D: Into<Cow<'a, str>>,
    {
        self.description = Some(description.into());
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

pub struct WithMetaSchemaBuilder<T: BaseTypeSchemaBuilder> {
    _phantom: PhantomData<T>,
}

impl<T: BaseTypeSchemaBuilder> BaseTypeSchemaBuilder for WithMetaSchemaBuilder<T> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder,
    {
        // Adding fields for the doc
        let identifier_ref = Identifier::build_schema(builder);
        let field_name = builder.add(TOption::new(identifier_ref).with_meta(NameDescription {
            name: "maybe_name",
            description: "An optional name for the type.",
        }));
        let description_ref = builder.add(
            TUnicode::try_new(
                DOC_MIN_LEN_UTF8_BYTES as u64,
                DOC_MAX_LEN_UTF8_BYTES as u64,
                LengthType::Utf8Byte,
            )
            .unwrap()
            .with_meta(NameDescription {
                name: "description",
                description: "Describes / documents the type. Use human readable text. \
                              No markup is allowed.",
            }),
        );
        let field_description =
            builder.add(TOption::new(description_ref).with_meta(NameDescription {
                name: "maybe_description",
                description: "Optional type description / documentation.",
            }));
        let uuid_ref = builder.add(TUuid::default().with_meta(NameDescription {
            name: "implements_uuid",
            description: "UUID to describe the conformance / implementation / \
                          protocol of this type uniquely.",
        }));
        let uuid_seq = builder.add(
            TSeq {
                meta: Meta::empty(),
                element: uuid_ref,
                length: U32IneRange::try_new("Doc type implements",
                                             MIN_IMPLEMENTS_ELEMENTS as u32,
                                             MAX_IMPLEMENTS_ELEMENTS as u32).unwrap(),
                ordering: seq::Ordering::Sorted {
                    direction: seq::Direction::Ascending,
                    unique: true,
                },
                multiple_of: None,
            }.with_meta(NameDescription{
                name: "implements",
                description: "A sequence of 'things' this type implements. What can this be used \
        for? Say for example your type is an ASCII type. With that information we can't \
        really say what ASCII is for. Say for example that ASCII has a special meaning in your \
        company: It's a part number. So you can give that ASCII type a UUID to declare 'this \
        type is a part number'. This makes it possible to find part numbers company wide." }));

        let field_implements = builder.add(TOption::new(uuid_seq).with_meta(NameOnly {
            name: "maybe_implements",
        }));

        let meta_struct = TStruct::default()
            .add(Field {
                name: Identifier::try_from("name").unwrap(),
                r#type: field_name,
            })
            .add(Field {
                name: Identifier::try_from("description").unwrap(),
                r#type: field_description,
            })
            .add(Field {
                name: Identifier::try_from("implements").unwrap(),
                r#type: field_implements,
            }).with_meta(NameDescription {
            name : "meta",
        description: "Meta information about the type. You can optionally specify a name, a description/\
        documentation and information about implementation/conformance."
        });
        let meta_ref = builder.add(meta_struct);

        let mut inner_struct = T::build_schema(builder);
        inner_struct.prepend(Field {
            name: Identifier::try_from("meta").unwrap(),
            r#type: meta_ref,
        });

        inner_struct
    }
}
