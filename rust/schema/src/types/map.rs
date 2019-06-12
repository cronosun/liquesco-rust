use crate::context::CmpContext;
use crate::context::KeyRefInfo;
use crate::context::ValidationContext;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::WithMetadata;
use crate::schema_builder::BuildsOwnSchema;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::types::boolean::TBool;
use crate::types::enumeration::TEnum;
use crate::types::enumeration::Variant;
use crate::types::key_ref::TKeyRef;
use crate::types::range::Inclusion;
use crate::types::range::TRange;
use crate::types::structure::Field;
use crate::types::structure::TStruct;
use crate::types::uint::TUInt;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::U32IneRange;
use liquesco_common::range::LqRangeBounds;
use liquesco_common::range::NewFull;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::core::LqReader;
use liquesco_serialization::types::seq::SeqHeader;
use serde::{Deserialize, Serialize};
use std::cmp::{min, Ordering};
use std::convert::TryFrom;

/// A map. Keys have to be unique. Has to be sorted by keys. The keys can optionally be referenced
/// to create recursive data structures.
///
/// Technical details: Internally a map looks like this:
/// `[[key1, value1], [key2, value2], ...]`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TMap<'a> {
    meta: Meta<'a>,
    key: TypeRef,
    value: TypeRef,
    length: U32IneRange,
    sorting: Sorting,
    anchors: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sorting {
    Ascending,
    Descending,
}

impl<'a> TMap<'a> {
    /// A new map; infinite length; Sorting: Ascending. No anchors.
    pub fn new(key: TypeRef, value: TypeRef) -> Self {
        Self {
            meta: Meta::default(),
            key,
            value,
            length: U32IneRange::full(),
            sorting: Sorting::Ascending,
            anchors: false,
        }
    }

    pub fn with_anchors(mut self, anchors: bool) -> Self {
        self.anchors = anchors;
        self
    }

    pub fn with_length(mut self, length: U32IneRange) -> Self {
        self.length = length;
        self
    }

    /// The type of keys in this map.
    pub fn key(&self) -> &TypeRef {
        &self.key
    }

    /// The type of values in this map.
    pub fn value(&self) -> &TypeRef {
        &self.value
    }

    /// Length constraints for the number of entries in this map.
    pub fn length(&self) -> &U32IneRange {
        &self.length
    }

    /// Sorting of keys in this map. Keys always have to be sorted in a map, either ascending
    /// (default) or descending.
    pub fn sorting(&self) -> Sorting {
        self.sorting
    }

    /// If this is true, the keys in this map can be referenced using key ref.
    pub fn anchors(&self) -> bool {
        self.anchors
    }
}

impl Type for TMap<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: ValidationContext<'c>,
    {
        let entries = SeqHeader::de_serialize(context.reader())?;
        let length = entries.length();

        // push ref info.
        if self.anchors {
            context.push_key_ref_info(KeyRefInfo::new(length));
        }

        validate_map(
            context,
            &self.length,
            length,
            &self.key,
            &self.value,
            self.sorting,
            self.anchors,
        )?;

        // pop ref info.
        if self.anchors {
            context.pop_key_ref_info()?;
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
        C: CmpContext<'c>,
    {
        compare_map(context, r1, r2, &self.key, &self.value)
    }

    fn reference(&self, index: usize) -> Option<&TypeRef> {
        match index {
            0 => Some(&self.key),
            1 => Some(&self.value),
            _ => None,
        }
    }

    fn set_reference(&mut self, index: usize, type_ref: TypeRef) -> Result<(), LqError> {
        match index {
            0 => {
                self.key = type_ref;
                Ok(())
            }
            1 => {
                self.value = type_ref;
                Ok(())
            }
            _ => LqError::err_new(format!("Map has no type at index {}", index)),
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
        B: SchemaBuilder<'static>,
    {
        let field_key = builder.add_unwrap(
            "map_key",
            TKeyRef::default().with_doc("Type of keys in this map."),
        );
        let field_value = builder.add_unwrap(
            "map_value",
            TKeyRef::default().with_doc("Type of values in this map."),
        );
        let length_element = builder.add_unwrap(
            "map_length_element",
            TUInt::try_new(0u32, u64::from(std::u32::MAX)).unwrap(),
        );
        let length_field = builder.add_unwrap(
            "map_length",
            TRange::new(length_element, Inclusion::BothInclusive, false).with_doc(
                "The length of a map (number of elements). Both - end and start - \
                 are included.",
            ),
        );
        let sorting_field = Sorting::build_schema(builder);
        let anchors_field = builder.add_unwrap(
            "anchors",
            TBool::default().with_doc(
                "If this is true, the keys in this map can be referenced using key refs. \
                 Note: Only values can reference keys. Keys cannot reference itself.",
            ),
        );

        TStruct::default()
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
            .add(Field::new(
                Identifier::try_from("anchors").unwrap(),
                anchors_field,
            ))
            .with_doc(
                "A sequence of key-value entries. Duplicate keys are not allowed. The keys \
                 can optionally be referenced to create recursive data structures.",
            )
    }
}

pub(crate) fn validate_map<'c, C>(
    context: &mut C,
    length_range: &U32IneRange,
    length: u32,
    key: &TypeRef,
    value: &TypeRef,
    sorting: Sorting,
    anchors: bool,
) -> Result<(), LqError>
where
    C: ValidationContext<'c>,
{
    // length OK?
    length_range.require_within(
        "Given length of map is outside bounds defined \
         in schema.",
        &length,
    )?;

    let wanted_ordering = match sorting {
        Sorting::Ascending => Ordering::Greater,
        Sorting::Descending => Ordering::Less,
    };

    let mut previous_key_reader: Option<C::Reader> = None;
    for index in 0..length {
        let entry_header = SeqHeader::de_serialize(context.reader())?;
        if entry_header.length() != 2 {
            return LqError::err_new(format!(
                "A map has to look like this: [[key1, \
                 value1], [key2, value2], ...]. So every key/value entry must be a sequence with \
                 two elements. The entry at index {} has {} elements.",
                index,
                entry_header.length()
            ));
        }

        // Create two copies (required for next iteration and for compare)
        let mut current_key_reader = context.reader().clone();
        let current_key_reader_for_next_iteration = context.reader().clone();

        if anchors {
            // This pop-push is required so keys cannot reference itself.
            let saved_info = context.pop_key_ref_info()?;
            context.validate(key)?;
            context.push_key_ref_info(saved_info);
        } else {
            context.validate(key)?;
        }
        context.validate(value)?;

        // Compare this key and the previous key to make sure keys have correct sorting
        // and there are no duplicates.
        if let Some(mut previous_reader) = previous_key_reader.take() {
            let key_cmp = context.compare(key, &mut current_key_reader, &mut previous_reader)?;
            if key_cmp != wanted_ordering {
                return LqError::err_new(format!(
                    "There's an ordering problem in the map. \
                     Keys have to be sorted according to the schema - and no duplicates are \
                     allowed. Compare result key at index {} to {}: {:?}; wanted {:?}.",
                    index - 1,
                    index,
                    key_cmp,
                    wanted_ordering
                ));
            }
        }

        previous_key_reader = Some(current_key_reader_for_next_iteration);
    }

    Ok(())
}

pub(crate) fn compare_map<'c, C>(
    context: &C,
    r1: &mut C::Reader,
    r2: &mut C::Reader,
    key: &TypeRef,
    value: &TypeRef,
) -> Result<Ordering, LqError>
where
    C: CmpContext<'c>,
{
    let entries1 = SeqHeader::de_serialize(r1)?;
    let entries2 = SeqHeader::de_serialize(r2)?;

    let min = min(entries1.length(), entries2.length());

    for _ in 0..min {
        // de-serialize both headers
        SeqHeader::de_serialize(r1)?;
        SeqHeader::de_serialize(r2)?;

        // compare keys
        let cmp_result = context.compare(key, r1, r2)?;
        if cmp_result != Ordering::Equal {
            return Ok(cmp_result);
        }

        // compare values
        let cmp_result = context.compare(value, r1, r2)?;
        if cmp_result != Ordering::Equal {
            return Ok(cmp_result);
        }
    }

    // ok, both are equal ... now length counts
    Ok(entries1.length().cmp(&entries2.length()))
}

impl BuildsOwnSchema for Sorting {
    fn build_schema<B>(builder: &mut B) -> TypeRef
    where
        B: SchemaBuilder<'static>,
    {
        builder.add_unwrap(
            "map_sorting",
            TEnum::default()
                .add_variant(Variant::new(Identifier::try_from("ascending").unwrap()))
                .add_variant(Variant::new(Identifier::try_from("descending").unwrap()))
                .with_doc(
                    "Determines the sort order of the keys in this map. You should usually \
                     use 'ascending' if not sure.",
                ),
        )
    }
}
