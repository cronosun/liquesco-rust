use crate::context::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::enumeration::TEnum;
use crate::enumeration::Variant;
use crate::identifier::Identifier;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::WithMetadata;
use crate::range::Inclusion;
use crate::range::TRange;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::structure::Field;
use crate::structure::TStruct;
use crate::uint::TUInt;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::U64IneRange;
use liquesco_common::range::LqRangeBounds;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::unicode::UncheckedUnicode;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::str::from_utf8;

/// A unicode text.
///
/// Note on encoding: The default encoding for unicode is UTF-8 - this is also the
/// only possible encoding supported in this implementation. The schema however
/// should not depend on the liquesco serialization - it should also be possible
/// to use other codecs (like legacy UTF-16).
///
/// Note: There's also the ascii type. If you want to transfer non-text strings,
/// this is usually what you want.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TUnicode<'a> {
    meta: Meta<'a>,
    length: U64IneRange,
    length_type: LengthType,
}

/// Specifies what the "length" of the unicode type means.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LengthType {
    /// This is the fastest possible way for validation: It just counts the number of bytes
    /// transferred. This is the same as `Utf8Byte` but only if the data has been
    /// encoded using UTF-8.
    ///
    /// Use this if the exact length is not so important but validation should be fast.
    Byte,

    /// This is the same as `Byte` when encoding is UTF-8. It's a bit harder to validate
    /// this when encoding is not UTF-8.
    Utf8Byte,

    /// Unicode Scalar Value: Counts the scalar values. This is not to be confused with
    /// grapheme clusters (which usually matches your idea of a character). Grapheme
    /// cluster length however has not been included since it's not supported in
    /// all systems out of the box.
    ScalarValue,
}

impl<'a> TUnicode<'a> {
    pub fn new(length: U64IneRange, length_type: LengthType) -> Self {
        Self {
            meta: Default::default(),
            length,
            length_type,
        }
    }

    pub fn try_new(
        min_length: u64,
        max_length: u64,
        length_type: LengthType,
    ) -> Result<Self, LqError> {
        Result::Ok(Self {
            meta: Meta::empty(),
            length: U64IneRange::try_new("Unicode length range", min_length, max_length)?,
            length_type,
        })
    }

    /// The length. Note: What "length" really means is defined by the `LengthType`.
    pub fn length(&self) -> &U64IneRange {
        &self.length
    }

    /// Defines what `length` means.
    pub fn length_type(&self) -> LengthType {
        self.length_type
    }
}

impl Type for TUnicode<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        // we read the unchecked unicode (so we know the length of utf-8 bytes)
        let bytes = UncheckedUnicode::de_serialize(context.reader())?;
        // this makes sure the data is valid UTF8
        let utf8_string = match from_utf8(bytes) {
            Result::Ok(value) => value,
            Result::Err(err) => {
                return LqError::err_new(format!("The given string is not valid UTF-8: {:?}", err));
            }
        };
        let length = match self.length_type {
            LengthType::Byte => bytes.len(),
            LengthType::Utf8Byte => bytes.len(),
            LengthType::ScalarValue => utf8_string.chars().count(),
        };
        let length_u64 = u64::try_from(length)?;
        self.length
            .require_within("Unicode schema validation (length)", &length_u64)?;

        Result::Ok(())
    }

    fn compare<'c, C>(
        &self,
        _: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>,
    {
        let bytes1 = UncheckedUnicode::de_serialize(r1)?;
        let bytes2 = UncheckedUnicode::de_serialize(r2)?;
        // lex compare
        Result::Ok(bytes1.cmp(&bytes2))
    }

    fn reference(&self, _: usize) -> Option<&TypeRef> {
        None
    }
}

impl WithMetadata for TUnicode<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TUnicode<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TUnicode<'_> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder<'static>,
    {
        let range_element = builder.add_unwrap(
            "unicode_length_element",
            TUInt::try_new(std::u64::MIN, std::u64::MAX)
                .unwrap()
        );
        let field_length = builder.add_unwrap(
            "unicode_length",
            TRange::new(range_element, Inclusion::BothInclusive, false)
        );

        let field_length_type = builder.add_unwrap(
            "length_type",
                TEnum::default()
                    .add(Variant::new(Identifier::try_from("byte").unwrap()))
                    .add(Variant::new(Identifier::try_from("utf8_byte").unwrap()))
                    .add(Variant::new(Identifier::try_from("scalar").unwrap()))
                    .with_doc(
                        "Specifies how the length of the unicode is measured. 'byte' can be \
                 measured very efficiently - but depends on the encoding. Note: Scalar must not be \
                 confused with unicode grapheme clusters.")
        );

        TStruct::default()
            .add(Field::new(
                Identifier::try_from("length").unwrap(),
                field_length,
            ))
            .add(Field::new(
                Identifier::try_from("length_type").unwrap(),
                field_length_type,
            ))
            .with_doc( "Arbitrary unicode text. This can be used for human readable text.")
    }
}
