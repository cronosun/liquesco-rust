use crate::boolean::TBool;
use crate::context::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::enumeration::TEnum;
use crate::enumeration::Variant;
use crate::identifier::Identifier;
use crate::key_ref::TKeyRef;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::WithMetadata;
use crate::option::TOption;
use crate::range::Inclusion;
use crate::range::TRange;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::structure::Field;
use crate::structure::TStruct;
use crate::uint::TUInt;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::U32IneRange;
use liquesco_common::range::LqRangeBounds;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::core::LqReader;
use liquesco_serialization::seq::SeqHeader;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// A sequence of 0-n elements where every element is of the same type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TSeq<'a> {
    meta: Meta<'a>,
    element: TypeRef,
    length: U32IneRange,
    ordering: Ordering,
    multiple_of: Option<u32>,
}

/// Requirements regarding ordering of the sequence.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ordering {
    /// No special ordering requirements.
    None,

    /// Elements in this sequence have to be sorted (and maybe also unique).
    ///
    /// Note: We use a dedicated struct (required for serde so the enum variant
    /// only has one value).
    Sorted(Sorted),
}

/// Ascending or descending.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    Ascending,
    Descending,
}

/// When sorted ordering is requires this specifies further details.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Sorted {
    pub direction: Direction,
    /// If this is true, no duplicate elements are allowed.
    pub unique: bool,
}

impl<'a> TSeq<'a> {
    /// New sequence without ordering.
    pub fn new(element: TypeRef, length: U32IneRange) -> Self {
        Self {
            meta: Default::default(),
            element,
            length,
            ordering: Ordering::None,
            multiple_of: None,
        }
    }

    /// New with no ordering.
    pub fn try_new(element: TypeRef, min_len: u32, max_len: u32) -> Result<Self, LqError> {
        Result::Ok(Self {
            meta: Meta::empty(),
            element,
            length: U32IneRange::try_new("", min_len, max_len)?,
            ordering: Ordering::None,
            multiple_of: Option::None,
        })
    }

    pub fn element(&self) -> &TypeRef {
        &self.element
    }

    pub fn length(&self) -> &U32IneRange {
        &self.length
    }

    /// Ordering requirements.
    pub fn ordering(&self) -> &Ordering {
        &self.ordering
    }

    /// Length must be a multiple of this value. Value must be >= 2.
    pub fn multiple_of(&self) -> Option<u32> {
        self.multiple_of
    }

    /// With ordering sorted.
    pub fn with_sorted(mut self, sorted: Sorted) -> Self {
        self.ordering = Ordering::Sorted(sorted);
        self
    }

    pub fn with_multiple_of(mut self, multiple_of: u32) -> Self {
        self.multiple_of = Some(multiple_of);
        self
    }
}

impl Type for TSeq<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let seq = SeqHeader::de_serialize(context.reader())?;
        let number_of_items = seq.length();

        self.length.require_within(
            "Sequence length validation \
             (schema; min/max elements in sequence)",
            &number_of_items,
        )?;

        // multiple of correct?
        if let Some(multiple_of) = self.multiple_of {
            if number_of_items % multiple_of != 0 {
                return LqError::err_new(format!(
                    "There's {:?} elements in this list. \
                     According to the schema the number of elements must be a multiple of {:?}.",
                    number_of_items, multiple_of
                ));
            }
        }

        match &self.ordering {
            Ordering::None => {
                // validate each element
                for _ in 0..number_of_items {
                    context.validate(&self.element)?;
                }
            }
            Ordering::Sorted(value) => {
                validate_with_ordering(
                    self,
                    context,
                    value.direction.clone(),
                    value.unique,
                    number_of_items,
                )?;
            }
        }

        Result::Ok(())
    }

    fn compare<'c, C>(
        &self,
        context: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<std::cmp::Ordering, LqError>
    where
        C: Context<'c>,
    {
        seq_compare(&self.element, context, r1, r2)
    }

    fn reference(&self, index: usize) -> Option<&TypeRef> {
        if index == 0 {
            Some(self.element())
        } else {
            None
        }
    }

    fn set_reference(&mut self, index: usize, type_ref: TypeRef) -> Result<(), LqError> {
        if index == 0 {
            self.element = type_ref;
            Ok(())
        } else {
            LqError::err_new(format!("Seq has no type at index {}", index))
        }
    }
}

impl WithMetadata for TSeq<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TSeq<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

#[inline]
pub(crate) fn seq_compare<'c, C>(
    element: &TypeRef,
    context: &C,
    r1: &mut C::Reader,
    r2: &mut C::Reader,
) -> Result<std::cmp::Ordering, LqError>
where
    C: Context<'c>,
{
    let header1 = SeqHeader::de_serialize(r1)?;
    let header2 = SeqHeader::de_serialize(r2)?;

    // it's very important that we finish reading to the end (see contract)
    let finish_reading =
        |header: SeqHeader, reader: &mut LqReader, num_read: u32| -> Result<(), LqError> {
            let len = header.length();
            if len > num_read {
                let missing = len - num_read;
                reader.skip_n_values_u32(missing)
            } else {
                Result::Ok(())
            }
        };

    // lex compare: first compare each element (even if their length is not equal)
    let min_to_read = header1.length().min(header2.length());
    for _ in 0..min_to_read {
        let cmp = context.compare(&element, r1, r2)?;
        if cmp != std::cmp::Ordering::Equal {
            // no need to finish to the end (see contract)
            return Result::Ok(cmp);
        }
    }

    // if their length are equal, we've read everything
    let result = header1.length().cmp(&header2.length());

    // here we have to finish to the end (if the result is `Equal`)
    if result == std::cmp::Ordering::Equal {
        finish_reading(header1, r1, min_to_read)?;
        finish_reading(header2, r2, min_to_read)?;
    }

    Result::Ok(result)
}

fn validate_with_ordering<'c, C>(
    this: &TSeq,
    context: &mut C,
    direction: Direction,
    unique: bool,
    number_of_items: u32,
) -> Result<(), LqError>
where
    C: Context<'c>,
{
    // here validation is a bit more complex
    let mut previous: Option<C::Reader> = Option::None;
    for idx in 0..number_of_items {
        // we need 3 readers (one for validation, one for this cmp and one for next cmp)
        let mut saved_reader1 = context.reader().clone();
        let saved_reader2 = context.reader().clone();
        context.validate(&this.element)?;

        if let Some(mut previous) = previous.take() {
            let equality = context.compare(&this.element, &mut previous, &mut saved_reader1)?;
            match equality {
                std::cmp::Ordering::Greater => {
                    // previous is greater: this is OK for descending lists
                    if direction != Direction::Descending {
                        return LqError::err_new(format!(
                            "Element at index {:?} is greater \
                             than element at index {:?}; this is OK but only for descending \
                             lists. This list is not a descending list.",
                            idx - 1,
                            idx
                        ));
                    }
                }
                std::cmp::Ordering::Less => {
                    // previous is less: this is OK for ascending lists
                    if direction != Direction::Ascending {
                        return LqError::err_new(format!(
                            "Element at index {:?} is less \
                             than element at index {:?}; this is OK but only for ascending \
                             lists. This list is not an ascending list.",
                            idx - 1,
                            idx
                        ));
                    }
                }
                std::cmp::Ordering::Equal => {
                    // this is only allowed when we accept duplicates
                    if unique {
                        return LqError::err_new(format!(
                            "Elements at index {:?} and {:?} in \
                             sequence are equal. This is not allowed, since sequence must not \
                             contain duplicates.",
                            idx - 1,
                            idx
                        ));
                    }
                }
            }
        }
        previous = Option::Some(saved_reader2);
    }

    Result::Ok(())
}

impl BaseTypeSchemaBuilder for TSeq<'_> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder<'static>,
    {
        let element_field = builder.add_unwrap(
            "element",
            TKeyRef::default().with_doc("The element type of a sequence."),
        );
        let length_element = builder.add_unwrap(
            "seq_length_element",
            TUInt::try_new(0, u64::from(std::u32::MAX)).unwrap(),
        );
        let length_field = builder.add_unwrap(
            "seq_length",
            TRange::new(length_element, Inclusion::BothInclusive, false).with_doc(
                "The length of a sequence (number of elements). It's tuple of start \
                 and end. Both - end and start - are included.",
            ),
        );

        let directed_enum = builder.add_unwrap(
            "direction",
            TEnum::default()
                .add(Variant::new(Identifier::try_from("ascending").unwrap()))
                .add(Variant::new(Identifier::try_from("descending").unwrap()))
                .with_doc(
                    "Determines how the elements in the sequence need to be sorted for \
                     the sequence to be valid.",
                ),
        );
        let unique = builder.add_unwrap(
            "unique",
            TBool::default().with_doc(
                "When this is true, no duplicate elements are allowed in the sequence. \
                 This automatically implies a sorted sequence.",
            ),
        );
        let sorted_struct = builder.add_unwrap(
            "sorting",
            TStruct::default()
                .add(Field::new(
                    Identifier::try_from("direction").unwrap(),
                    directed_enum,
                ))
                .add(Field::new(Identifier::try_from("unique").unwrap(), unique))
                .with_doc(
                    "Determines how the sequence needs to be sorted and whether duplicate \
                     elements are allowed.",
                ),
        );
        let ordering_field = builder.add_unwrap(
            "ordering",
            TEnum::default()
                .add(Variant::new(Identifier::try_from("none").unwrap()))
                .add(
                    Variant::new(Identifier::try_from("sorted").unwrap()).add_value(sorted_struct),
                ).with_doc(
                 "Ordering is optional. When there's no ordering it can be irrelevant or \
        ordering has a special meaning. It's also possible to specify a required sorting (in this \
        case it's also possible to disallow duplicates)."));

        let multiple_of = builder.add_unwrap(
            "multiple_of",
            TUInt::try_new(2, u64::from(std::u32::MAX))
                .unwrap()
                .with_doc(
                "It's possible to specify another requirement on the length of the list \
                 (number of elements). If this is for example 2, only lengths of 0, 2, 4, 6, 8, \
                 ... are allowed.",
            ),
        );
        let multiple_of_field = builder.add_unwrap("maybe_multiple_of", TOption::new(multiple_of));

        TStruct::default()
            .add(Field::new(
                Identifier::try_from("element").unwrap(),
                element_field,
            ))
            .add(Field::new(
                Identifier::try_from("length").unwrap(),
                length_field,
            ))
            .add(Field::new(
                Identifier::try_from("ordering").unwrap(),
                ordering_field,
            ))
            .add(Field::new(
                Identifier::try_from("multiple_of").unwrap(),
                multiple_of_field,
            ))
            .with_doc("A sequence contains 0-n elements of the same type.")
    }
}
