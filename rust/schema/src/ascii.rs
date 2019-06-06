use crate::context::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::WithMetadata;
use crate::range::Inclusion;
use crate::range::TRange;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::seq::Direction::Ascending;
use crate::seq::Sorted;
use crate::seq::TSeq;
use crate::structure::Field;
use crate::structure::TStruct;
use crate::uint::TUInt;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::{U32IneRange, U64IneRange};
use liquesco_common::range::LqRangeBounds;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::unicode::UncheckedUnicode;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::ops::Index;
use crate::context::CmpContext;

/// Text within the ASCII code range (0 to 127). Can have length restriction and code
/// range restriction.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TAscii<'a> {
    meta: Meta<'a>,
    length: U64IneRange,
    codes: CodeRange,
}

const CODE_RANGE_ELEMENTS_MIN: usize = 2;
const CODE_RANGE_ELEMENTS_MAX: usize = 64;

/// It's always a tuple of 2 values (min inclusive and max exclusive). Each value is unique
/// and it's ordered ascending. E.g. [10, 30, 50, 60] means that codes 10-29 (inclusive) and
/// codes 50-59 (inclusive) are allowed.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CodeRange(Vec<u8>);

impl CodeRange {
    pub fn try_new(min: u8, max: u8) -> Result<CodeRange, LqError> {
        let mut range = CodeRange(Vec::new());
        range.add(min, max)?;
        Ok(range)
    }

    pub fn add(&mut self, min: u8, max: u8) -> Result<(), LqError> {
        if min >= max {
            return LqError::err_new(format!(
                "Cannot add code range. Contract: min<max. \
                 Have min {:?}, max {:?}.",
                min, max
            ));
        }
        // note: yes, it's >128. Highest value us 128, since the end is not inclusive.
        if max > 128 {
            return LqError::err_new(format!(
                "Ascii code range: max must be <= 128. Have {:?}.",
                max
            ));
        }
        let len = self.0.len();
        if len > CODE_RANGE_ELEMENTS_MAX {
            return LqError::err_new("Too many elements in ascii code range.");
        }
        // strict ordering
        if len > 0 {
            let previous = self.0[len - 1];
            if min <= previous {
                return LqError::err_new(format!(
                    "Ascii code range list must be ordered \
                     (ascending and no duplicates). Previous element is {:?}, you're trying \
                     to add {:?}.",
                    previous, min
                ));
            }
        }
        self.0.push(min);
        self.0.push(max);
        Ok(())
    }

    pub fn contains(&self, item: u8) -> bool {
        let len = self.0.len();
        for tuple_index in 0..(len / 2) {
            let min = self.0[tuple_index * 2];
            let max_exclusive = self.0[tuple_index * 2 + 1];
            if item >= min && item < max_exclusive {
                return true;
            }
        }
        false
    }

    pub fn contains_chr(&self, chr: char) -> bool {
        let u32_chr = chr as u32;
        if u32_chr <= 255 {
            self.contains(u32_chr as u8)
        } else {
            // can never contain (since clearly outside ascii range)
            false
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Index<usize> for CodeRange {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl TAscii<'_> {
    pub fn new(length: U64IneRange, codes: CodeRange) -> Self {
        Self {
            meta: Meta::empty(),
            length,
            codes,
        }
    }

    pub fn try_new(
        min_length: u64,
        max_length: u64,
        min_code: u8,
        max_code: u8,
    ) -> Result<Self, LqError> {
        Result::Ok(Self {
            meta: Meta::empty(),
            length: U64IneRange::try_new("Ascii length range", min_length, max_length)?,
            // note: we add 1 since this here is inclusive.
            codes: CodeRange::try_new(min_code, max_code + 1)?,
        })
    }

    /// Allowed ascii code range(s).
    pub fn codes(&self) -> &CodeRange {
        &self.codes
    }

    /// Minimum / maximum number of bytes (which is at the same time also number
    /// of ASCII characters).
    pub fn length(&self) -> &U64IneRange {
        &self.length
    }
}

impl Type for TAscii<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let bytes = UncheckedUnicode::de_serialize(context.reader())?;

        // first check length (that's faster)
        let length = bytes.len();
        let length_u64 = u64::try_from(length)?;
        self.length.require_within(
            "Ascii schema validation (length; bytes; \
             number of characters)",
            &length_u64,
        )?;

        // now we have to check each character
        for byte in bytes {
            let contains = self.codes.contains(*byte);
            if !contains {
                return LqError::err_new(format!("The given ascii string contains a character \
                that's not within the allowed code range. Ascii code is {:?}; code ranges is {:?}; \
                note: it's a list of pairs (min; max exclusive).", byte, self.codes));
            }
        }

        Result::Ok(())
    }

    fn compare<'c, C>(
        &self,
        _: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: CmpContext<'c>,
    {
        let bytes1 = UncheckedUnicode::de_serialize(r1)?;
        let bytes2 = UncheckedUnicode::de_serialize(r2)?;
        // lex compare
        Result::Ok(bytes1.cmp(&bytes2))
    }

    fn reference(&self, _: usize) -> Option<&TypeRef> {
        None
    }

    fn set_reference(&mut self, _: usize, _: TypeRef) -> Result<(), LqError> {
        LqError::err_new("This type has no references")
    }
}

impl<'a> WithMetadata for TAscii<'a> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TAscii<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TAscii<'_> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder<'static>,
    {
        let length_element = builder.add_unwrap(
            "ascii_length_element",
            TUInt::try_new(0, std::u64::MAX).unwrap(),
        );
        let field_length = builder.add_unwrap(
            "ascii_length",
            TRange::new(length_element, Inclusion::BothInclusive, false).with_doc(
                "The length constraint of the ASCII string (also number of bytes). \
                 Start and end \
                 are both inclusive.",
            ),
        );

        let range_element = builder.add_unwrap("ascii_code", TUInt::try_new(0, 128).unwrap());
        let field_codes = builder.add_unwrap(
            "codes",
            TSeq::new(
                range_element,
                U32IneRange::try_new(
                    "Code range",
                    CODE_RANGE_ELEMENTS_MIN as u32,
                    CODE_RANGE_ELEMENTS_MAX as u32,
                )
                .unwrap(),
            )
            .with_sorted(Sorted {
                direction: Ascending,
                unique: true,
            })
            .with_multiple_of(2)
            .with_doc(
                "Use this sequence to supply allowed ASCII code ranges. It takes pairs of ASCII \
                 codes (start, end); start is inclusive, end is exclusive. The ASCII string is \
                 valid if every character of the ASCII string is within one of those ranges.",
            ),
        );

        TStruct::default()
            .add(Field::new(
                Identifier::try_from("length").unwrap(),
                field_length,
            ))
            .add(Field::new(
                Identifier::try_from("codes").unwrap(),
                field_codes,
            ))
            .with_doc(
                "The ascii type must not be used to transfer human readable text. It's to be \
                 used to transfer machine readable strings. Only characters withing the ASCII \
                 range are allowed.",
            )
    }
}
