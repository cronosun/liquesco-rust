use std::cmp::Ordering;
use std::convert::TryFrom;
use liquesco_serialization::boolean::Bool;
use serde::{Deserialize, Serialize};

use liquesco_common::error::LqError;
use liquesco_common::range::LqRangeBounds;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::sint::SInt64;

use crate::core::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::doc_type::DocType;
use crate::identifier::Identifier;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::seq::{Direction, TSeq};
use crate::seq::Ordering as SeqOrdering;
use crate::structure::Field;
use crate::structure::TStruct;
use crate::boolean::TBool;
use crate::reference::TReference;
use crate::enumeration::TEnum;
use crate::enumeration::Variant;
use liquesco_serialization::seq::SeqHeader;
use std::cmp::Ordering::Equal;
use liquesco_serialization::core::LqReader;

/// A range. Constraints:
///  - start <= end (if allow_equal) or start < end (if !allow_equal).
#[derive(new, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TRange {
    /// The element in the range.
    pub element: TypeRef,
    pub inclusion: Inclusion,
    /// If this is true, we allow `start`==`end`.
    pub allow_equal: bool,
}

#[derive(new, Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Inclusion {
    BothInclusive,
    StartInclusive,
    BothExclusive,
    EndInclusive,
    /// Exclusion / inclusion is provided in the data.
    Supplied,
}

impl TRange {
    pub fn inclusion(&self) -> Inclusion {
        self.inclusion
    }

    pub fn element(&self) -> TypeRef {
        self.element
    }

    pub fn allow_equal(&self) -> bool {
        self.allow_equal
    }
}

impl Inclusion {
    fn try_from(ordinal : u8) -> Option<Inclusion> {
        match ordinal {
            0 => Some(Inclusion::BothInclusive),
            1 => Some(Inclusion::StartInclusive),
            2 => Some(Inclusion::BothExclusive),
            3 => Some(Inclusion::EndInclusive),
            4 => Some(Inclusion::Supplied),
            _ => None
        }
    }
}

impl Type for TRange {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
        where
            C: Context<'c>,
    {
        let seq = SeqHeader::de_serialize(context.reader())?;
        let number_of_items = seq.length();

        let expected_number_of_items = if self.inclusion==Inclusion::Supplied {
            4
        } else {
            2
        };

        if number_of_items!=expected_number_of_items {
            return LqError::err_new(format!("The given range has a seq length of {}, we \
            need a length of {} (start, end and maybe 2 more items with information about \
            inclusion).", number_of_items, expected_number_of_items));
        }

        // start
        let mut start_reader = context.reader().clone();
        context.validate(self.element);
        // end
        let mut end_reader = context.reader().clone();
        context.validate(self.element);

        let inclusive : (bool, bool) = match self.inclusion {
            Inclusion::Supplied => {
                (Bool::de_serialize(context.reader())?,  Bool::de_serialize(context.reader())?)
            },
            Inclusion::BothInclusive => (true, true),
            Inclusion::StartInclusive => (true, false),
            Inclusion::BothExclusive => (false, false),
            Inclusion::EndInclusive => (false, true),
        };

        // Now compare start and end
        let cmp = context.compare(self.element,&mut start_reader, &mut end_reader)?;
        match cmp {
            Ordering::Greater => {
                LqError::err_new("The given start (first element) is greater then \
                given end (second element) in range. Start can never be greater than end.")
            }
            Ordering::Equal => {
                if self.allow_equal {
                    Ok(())
                } else {
                    LqError::err_new("Start (first element) is equal to end (second element). \
                    This is not allowed according to the schema (see 'allow_equal').")
                }
            },
            Ordering::Less => {
                Ok(())
            }
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
        let header1 = SeqHeader::de_serialize(r1)?;
        let header2 = SeqHeader::de_serialize(r2)?;

        if header1.length()!=header2.length() {
            return LqError::err_new("Ranges with different length (cannot compare them).")
        }

        let with_inclusion = header1.length()==4;

        let cmp1 = context.compare(self.element,r1, r2)?;
        Ok(if cmp1!=Equal {
            cmp1
        } else {
            let cmp2 = context.compare(self.element,r1, r2)?;
            if cmp2!=Equal {
                cmp2
            } else {
                if with_inclusion {
                    let cmp3 = Bool::de_serialize(r1)?.cmp(&Bool::de_serialize(r2)?);
                    if cmp3!=Equal {
                        cmp3
                    } else {
                        Bool::de_serialize(r1)?.cmp(&Bool::de_serialize(r2)?)
                    }
                } else {
                    Equal
                }
            }
        })
    }

    fn reference(&self, index: usize) -> Option<TypeRef> {
        if index == 0 {
            Some(self.element)
        } else {
            None
        }
    }
}


impl BaseTypeSchemaBuilder for TRange {
    fn build_schema<B>(builder: &mut B) -> DocType<'static, TStruct<'static>>
        where
            B: SchemaBuilder,
    {
        let element_field = builder.add(
            DocType::from(TReference::default())
                .with_name_unwrap("range_element")
                .with_description("The start and end type of the range."),
        );

        let inclusion_field = builder.add(
            DocType::from(
                TEnum::default()
                    .add(Variant::new(Identifier::try_from("both_inclusive").unwrap()))
                    .add(Variant::new(Identifier::try_from("start_inclusive").unwrap()))
                    .add(Variant::new(Identifier::try_from("both_exclusive").unwrap()))
                    .add(Variant::new(Identifier::try_from("end_inclusive").unwrap()))
                    .add(Variant::new(Identifier::try_from("supplied").unwrap()))
            )
                .with_name_unwrap("inclusion")
                .with_description(
                    "Determines whether start and end are inclusive. There's one \
                    special value: 'Supplied'. When you choose this, the data has to contain \
                    the information whether start/end are inclusive or not.",
                ),
        );

        let allow_equal_field = builder.add(DocType::from(TBool::default())
            .with_name_unwrap("allow_equal")
            .with_description("General rule is start < end. If this value is true, \
            start == end is also allowed."));

        // just an empty struct (but more fields will be added by the system)
        DocType::from(
            TStruct::default()
                .add(Field::new(
                    Identifier::try_from("element").unwrap(),
                    element_field,
                ))
                .add(Field::new(
                    Identifier::try_from("inclusion").unwrap(),
                    inclusion_field,
                ))
                .add(Field::new(
                    Identifier::try_from("allow_equal").unwrap(),
                    allow_equal_field,
                ))
        )
            .with_name_unwrap("range")
            .with_description("A sequence contains 0-n elements of the same type.")
    }
}
