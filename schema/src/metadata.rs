use crate::identifier::Identifier;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::U32IneRange;
use liquesco_serialization::types::uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::convert::TryFrom;

use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::types::option::TOption;
use crate::types::seq;
use crate::types::seq::{Sorted, TSeq};
use crate::types::structure::Field;
use crate::types::structure::TStruct;
use crate::types::unicode::{LengthType, TUnicode};
use crate::types::uuid::TUuid;
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

    fn with_doc<D: Into<Cow<'m, str>>>(mut self, doc: D) -> Self
    where
        Self: Sized,
    {
        self.set_meta(Meta {
            doc: Some(doc.into()),
            implements: self.meta().implements.clone(),
        });
        self
    }
}

/// What information a type contains.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Information {
    /// Only the pure type, no meta data; no documentation and no conformance.
    Type,

    /// Contains only the technical information used to process the type; contains no
    /// documentation but contains conformance.
    Technical,

    /// Full type; contains documentation and conformance. Usually used to generate documentation.
    Full,
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
        if self.implements.is_none() {
            self.implements = Option::Some(Implements::try_new(&[uuid.into()])?);
        } else {
            let mut implements = self.implements.take().unwrap();
            implements.add(uuid.into())?;
            self.implements = Some(implements);
        }
        Ok(())
    }

    pub fn information(&self) -> Information {
        if let None = self.doc() {
            if self.implements().is_empty() {
                Information::Type
            } else {
                Information::Technical
            }
        } else {
            Information::Full
        }
    }

    /// Reduces information to given level. Returns the new `Meta` if information has been reduced.
    /// Returns `None` if there's no need to reduce information (`self` has not more information).
    pub fn reduce_information(&self, information: Information) -> Option<Meta<'a>> {
        let given = self.information();
        match information {
            Information::Full => None,
            Information::Technical => match given {
                Information::Full => Some(Meta {
                    doc: None,
                    implements: self.implements.clone(),
                }),
                Information::Type | Information::Technical => None,
            },
            Information::Type => match given {
                Information::Full | Information::Technical => Some(Meta {
                    doc: None,
                    implements: None,
                }),
                Information::Type => None,
            },
        }
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
    /// New implements. Duplicate elements are removed. Needs at least one element.
    pub fn try_new(implements: &[Uuid]) -> Result<Self, LqError> {
        let mut new_vec = Vec::from(implements);
        // always needs to be sorted
        new_vec.sort();
        new_vec.dedup();

        let number = new_vec.len();
        if number < MIN_IMPLEMENTS_ELEMENTS {
            LqError::err_new("You need at least one element in 'implements'.")
        } else if number > MAX_IMPLEMENTS_ELEMENTS {
            LqError::err_new(format!(
                "There are too many implements elements. Maximum is {:?}; got {:?} elements.",
                MAX_IMPLEMENTS_ELEMENTS, number
            ))
        } else {
            Ok(Implements(new_vec))
        }
    }

    /// Adds a new 'implements' - does nothing if 'implements' has already been added.
    pub fn add(&mut self, implements: Uuid) -> Result<(), LqError> {
        let number = self.0.len() + 1;
        if number > MAX_IMPLEMENTS_ELEMENTS {
            LqError::err_new(format!(
                "There are too many implements elements. Maximum is {:?}; got {:?} elements.",
                MAX_IMPLEMENTS_ELEMENTS, number
            ))
        } else {
            if self.0.contains(&implements) {
                return Ok(());
            }
            self.0.push(implements);
            // always needs to be sorted ascending
            self.0.sort();
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
            .with_doc(
                "Describes / documents the type. Use human readable text. \
                 No markup is allowed.",
            ),
        );
        let field_doc = builder.add_unwrap(
            "maybe_doc",
            TOption::new(doc_ref).with_doc("Optional type description / documentation."),
        );
        let uuid_ref = builder.add_unwrap(
            "implements_uuid",
            TUuid::default().with_doc(
                "UUID to describe the conformance / implementation / \
                 protocol of this type uniquely.",
            ),
        );
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

        let field_implements = builder.add_unwrap("maybe_implements", TOption::new(uuid_seq));

        let meta_struct = TStruct::default()
            .add(Field::new(Identifier::try_from("doc").unwrap(), field_doc))
            .add(Field::new(
                Identifier::try_from("implements").unwrap(),
                field_implements,
            ))
            .with_doc(
                "Meta information about the type. You can optionally provide a description/\
                 documentation and information about implementation/conformance.",
            );
        let meta_ref = builder.add_unwrap("meta", meta_struct);

        let mut inner_struct = T::build_schema(builder);
        inner_struct.prepend(Field::new(Identifier::try_from("meta").unwrap(), meta_ref));

        inner_struct
    }
}
