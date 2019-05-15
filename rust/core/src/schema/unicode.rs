use crate::common::error::LqError;
use crate::common::ine_range::U64IneRange;
use crate::common::range::LqRangeBounds;
use crate::schema::core::Context;
use crate::schema::core::Type;
use crate::serialization::core::DeSerializer;
use crate::serialization::unicode::UncheckedUnicode;
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
#[derive(Clone, Debug)]
pub struct TUnicode {
    /// The length. Note: What "length" really means is defined by the `LengthType`.
    pub length: U64IneRange,
    /// Defines what `length` means.
    pub length_type: LengthType,
}

#[derive(Clone, Debug)]
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

impl TUnicode {
    pub fn try_new(
        min_length: u64,
        max_length: u64,
        length_type: LengthType,
    ) -> Result<Self, LqError> {
        Result::Ok(Self {
            length: U64IneRange::try_new_msg("Unicode length range", min_length, max_length)?,
            length_type,
        })
    }
}

impl<'a> Type<'a> for TUnicode {
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
}
