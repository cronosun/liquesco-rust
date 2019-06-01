use crate::context::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::NameDescription;
use crate::metadata::WithMetadata;
use crate::metadata::NameOnly;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::structure::TStruct;
use crate::structure::Field;
use crate::reference::TReference;
use crate::uint::TUInt;
use crate::range::TRange;
use crate::boolean::TBool;
use crate::range::Inclusion;
use crate::enumeration::TEnum;
use crate::enumeration::Variant;
use crate::identifier::Identifier;
use liquesco_common::error::LqError;
use liquesco_serialization::core::DeSerializer;
use serde::{Deserialize, Serialize};
use std::cmp::{Ordering, min};
use liquesco_serialization::seq::SeqHeader;
use liquesco_common::ine_range::U32IneRange;
use liquesco_common::range::LqRangeBounds;
use liquesco_common::range::NewFull;
use liquesco_serialization::core::LqReader;
use std::convert::TryFrom;

/// A map. Keys have to be unique. Has to be sorted by keys. The keys can optionally be referenced
/// to create recursive data structures.
///
/// Technical details: Internally a map looks like this:
/// `[[key1, value1], [key2, value2], ...]`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TMap<'a> {
    meta: Meta<'a>,
    key : TypeRef,
    value : TypeRef,
    length : U32IneRange,
    sorting : Sorting,
    anchors : bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sorting {
    Ascending,
    Descending
}

impl TMap<'_> {

    /// A new map; infinite length; Sorting: Ascending. No anchors.
    pub fn new(key : TypeRef, value : TypeRef) -> Self {
        Self {
            meta : Meta::default(),
            key,
            value,
            length : U32IneRange::full(),
            sorting : Sorting::Ascending,
            anchors : false,
        }
    }

    pub fn with_anchors(mut self, anchors : bool) -> Self {
        self.anchors = anchors;
        self
    }
}


impl Type for TMap<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
        where
            C: Context<'c>,
    {
        let entries = SeqHeader::de_serialize(context.reader())?;
        let length = entries.length();

        // length OK?
        self.length.require_within("Given length of map is outside bounds defined \
        in schema.", &length)?;

        let wanted_ordering = match self.sorting {
            Sorting::Ascending => Ordering::Greater,
            Sorting::Descending => Ordering::Less,
        };


        // persist ref info (when we have nested maps)
        let persisted_ref_info = if self.anchors {
            let persisted = context.key_ref_info().clone();
            context.key_ref_info().set_map_len(Some(length));
            Some(persisted)
        } else {
            None
        };

        let mut previous_key_reader : Option<C::Reader> = None;
        for index in 0..length {
            let entry_header = SeqHeader::de_serialize(context.reader())?;
            if entry_header.length()!=2 {
                return LqError::err_new(format!("A map has to look like this: [[key1, \
                value1], [key2, value2], ...]. So every key/value entry must be a sequence with \
                two elements. The entry at index {} has {} elements.",
                index, entry_header.length()));
            }

            // Create two copies (required for next iteration and for compare)
            let mut current_key_reader = context.reader().clone();
            let current_key_reader_for_next_iteration = context.reader().clone();
            context.validate(self.key)?;
            context.validate(self.value)?;

            // Compare this key and the previous key to make sure keys have correct sorting
            // and there are no duplicates.
            if let Some(mut previous_reader) = previous_key_reader.take() {
                let key_cmp = context.compare(
                    self.key, &mut current_key_reader, &mut previous_reader)?;
                if key_cmp!=wanted_ordering {
                    return LqError::err_new(format!("There's an ordering problem in the map. \
                    Keys have to be sorted according to the schema - and no duplicates are \
                    allowed. Compare result key at index {} to {}: {:?}; wanted {:?}.",
                    index-1, index, key_cmp, wanted_ordering));
                }
            }

            previous_key_reader = Some(current_key_reader_for_next_iteration);
        }

        // maybe restore key ref info (if we have nested maps)
        if let Some(persisted) = persisted_ref_info {
            context.key_ref_info().restore_from(persisted);
        }

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
        let entries1 = SeqHeader::de_serialize(r1)?;
        let entries2 = SeqHeader::de_serialize(r2)?;

        let min = min(entries1.length(), entries2.length());

        for _ in 0..min {
            // de-serialize both headers
            SeqHeader::de_serialize(r1)?;
            SeqHeader::de_serialize(r2)?;

            // compare keys
            let cmp_result = context.compare(self.key, r1, r2)?;
            if cmp_result!=Ordering::Equal {
                return Ok(cmp_result);
            }

            // compare values
            let cmp_result = context.compare(self.key, r1, r2)?;
            if cmp_result!=Ordering::Equal {
                return Ok(cmp_result);
            }
        }

        // ok, both are equal ... now length counts
        Ok(entries1.length().cmp(&entries2.length()))
    }

    fn reference(&self, index: usize) -> Option<TypeRef> {
        match index {
            0 => Some(self.key),
            1 => Some(self.value),
            _ => None
        }
    }
}

impl WithMetadata for TMap<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TMap<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TMap<'_> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
        where
            B: SchemaBuilder,
    {
        let field_key = builder.add(TReference::default()
            .with_meta(NameDescription {
            name: "key",
            doc: "Type of keys in this map.",
        }));
        let field_value = builder.add(TReference::default()
            .with_meta(NameDescription {
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
            TRange::new(length_element, Inclusion::BothInclusive, false)
                .with_meta(
                NameDescription {
                    name: "map_length",
                    doc: "The length of a map (number of elements). Both - end and start - \
                    are included.",
                },
            ),
        );
        let sorting_field = builder.add(
            TEnum::default()
                .add(Variant::new(Identifier::try_from("ascending").unwrap()))
                .add(Variant::new(Identifier::try_from("descending").unwrap()))
                .with_meta(NameDescription {
                    name: "sorting",
                    doc: "Determines the sort order of the keys in this map. You should usually \
                    use 'ascending' if not sure.",
                }),
        );
        let anchors_field = builder.add(TBool::default().with_meta(
            NameDescription {
                name: "anchors",
                doc: "If this is true, the keys in this map can be referenced using key refs.",
            }
        ));

        TStruct::default()
            .add(Field::new(
                Identifier::try_from("key").unwrap(),
                field_key,
            ))
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
            .add(Field::new(
                Identifier::try_from("anchors").unwrap(),
                anchors_field,
            ))
            .with_meta(NameDescription {
                name: "map",
                doc: "A sequence of key-value entries. Duplicate keys are not allowed. The keys \
                can optionally be referenced to create recursive data structures.",
            })
    }
}
