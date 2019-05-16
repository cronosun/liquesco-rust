use crate::common::error::LqError;
use crate::common::ine_range::U64IneRange;
use crate::common::ine_range::U8IneRange;
use crate::schema::core::Context;
use crate::schema::core::Type;
use crate::serialization::core::DeSerializer;
use crate::serialization::unicode::UncheckedUnicode;
use std::cmp::Ordering;
use std::convert::TryFrom;
use crate::common::range::LqRangeBounds;

#[derive(Clone, Debug)]
pub struct TAscii {
    /// Minimum / maximum number of bytes (which is at the same time also number 
    /// of ASCII characters)
    pub length: U64IneRange,
    /// Allowed ascii code range (note: this field is private since needs further validation)
    codes: U8IneRange,
}

impl TAscii {
    pub fn try_new(
        min_length: u64,
        max_length: u64,
        min_code: u8,
        max_code: u8,
    ) -> Result<TAscii, LqError> {
        if min_code > 127 || max_code > 127 {
            return LqError::err_new(format!(
                "Min ascii code / max ascii code must be within the ascii range (0-127; \
                 inclusive); have {:?} - {:?}.",
                min_code, max_code
            ));
        }

        Result::Ok(Self {
            length: U64IneRange::try_new_msg("Ascii length range", min_length, max_length)?,
            codes: U8IneRange::try_new_msg("Ascii code range", min_code, max_code)?,
        })
    }

    /// Allowed ascii code range
    pub fn codes(&self) -> &U8IneRange {
        &self.codes
    }
}

impl<'a> Type for TAscii {
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
            self.codes.require_within(
                "Ascii schema validation (allowed \
                 ascii codes)",
                byte,
            )?;
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
        C: Context<'c>,
    {
        let bytes1 = UncheckedUnicode::de_serialize(r1)?;
        let bytes2 = UncheckedUnicode::de_serialize(r2)?;
        // lex compare
        Result::Ok(bytes1.cmp(&bytes2))
    }
}
