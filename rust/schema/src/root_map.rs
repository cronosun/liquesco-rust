use crate::context::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::map::compare_map;
use crate::map::validate_map;
use crate::map::Sorting;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::NameDescription;
use crate::metadata::NameOnly;
use crate::metadata::WithMetadata;
use crate::range::Inclusion;
use crate::range::TRange;
use crate::reference::TReference;
use crate::schema_builder::BuildsOwnSchema;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::structure::Field;
use crate::structure::TStruct;
use crate::uint::TUInt;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::U32IneRange;
use liquesco_common::range::NewFull;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::seq::SeqHeader;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

/// A map with a root. Keys have to be unique. The keys can be referenced. The root cannot be
/// referenced. The root can reference keys.
///
/// Technical details: Internally a root map looks like this:
/// `[[[key1, value1], [key2, value2], ...], root]`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TRootMap<'a> {
    meta: Meta<'a>,
    root: TypeRef,
    key: TypeRef,
    value: TypeRef,
    length: U32IneRange,
    sorting: Sorting,
}

impl TRootMap<'_> {
    /// A new map; infinite length; Sorting: Ascending.
    pub fn new(root: TypeRef, key: TypeRef, value: TypeRef) -> Self {
        Self {
            meta: Meta::default(),
            root,
            key,
            value,
            length: U32IneRange::full(),
            sorting: Sorting::Ascending,
        }
    }
}

impl Type for TRootMap<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let outer_seq = SeqHeader::de_serialize(context.reader())?;
        if outer_seq.length() != 2 {
            return LqError::err_new(format!(
                "A root map has to look like this: [[[key1, \
                 value1], [key2, value2], ...], root]]. So the outer sequence has to have \
                 exactly 2 elements. Have {} elements.",
                outer_seq.length()
            ));
        }

        let entries = SeqHeader::de_serialize(context.reader())?;
        let length = entries.length();

        // persist ref info (when we have nested maps)
        let persisted_ref_info = context.key_ref_info().clone();
        context.key_ref_info().set_map_len(Some(length));

        validate_map(
            context,
            &self.length,
            length,
            self.key,
            self.value,
            self.sorting,
        )?;

        // now validate the root
        context.validate(self.root)?;

        // maybe restore key ref info (if we have nested maps)
        context.key_ref_info().restore_from(persisted_ref_info);

        Ok(())
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
        // the outer sequence
        SeqHeader::de_serialize(r1)?;
        SeqHeader::de_serialize(r2)?;

        let cmp_result = compare_map(context, r1, r2, self.key, self.value)?;
        if cmp_result == Ordering::Equal {
            // continue... also compare root
            context.compare(self.root, r1, r2)
        } else {
            Ok(cmp_result)
        }
    }

    fn reference(&self, index: usize) -> Option<TypeRef> {
        match index {
            0 => Some(self.root),
            1 => Some(self.key),
            2 => Some(self.value),
            _ => None,
        }
    }
}

impl WithMetadata for TRootMap<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TRootMap<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TRootMap<'_> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder,
    {
        let field_root = builder.add(TReference::default().with_meta(NameDescription {
            name: "root",
            doc: "The root type in this map.",
        }));
        let field_key = builder.add(TReference::default().with_meta(NameDescription {
            name: "key",
            doc: "Type of keys in this map.",
        }));
        let field_value = builder.add(TReference::default().with_meta(NameDescription {
            name: "value",
            doc: "Type of values in this map.",
        }));
        let length_element = builder.add(
            TUInt::try_new(0, std::u32::MAX as u64)
                .unwrap()
                .with_meta(NameOnly {
                    name: "map_length_element",
                }),
        );
        let length_field = builder.add(
            TRange::new(length_element, Inclusion::BothInclusive, false).with_meta(
                NameDescription {
                    name: "map_length",
                    doc: "The length of a map (number of elements). Both - end and start - \
                          are included.",
                },
            ),
        );
        let sorting_field = Sorting::build_schema(builder);

        TStruct::default()
            .add(Field::new(
                Identifier::try_from("root").unwrap(),
                field_root,
            ))
            .add(Field::new(Identifier::try_from("key").unwrap(), field_key))
            .add(Field::new(
                Identifier::try_from("value").unwrap(),
                field_value,
            ))
            .add(Field::new(
                Identifier::try_from("length").unwrap(),
                length_field,
            ))
            .add(Field::new(
                Identifier::try_from("sorting").unwrap(),
                sorting_field,
            ))
            .with_meta(NameDescription {
                name: "root_map",
                doc: "A map with a root. Keys have to be unique. The keys can be referenced. \
                      The root cannot be referenced. The root can reference keys.",
            })
    }
}
