use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Validator;
use crate::schema::validators::Validators;
use crate::serialization::core::DeSerializer;
use crate::serialization::tunicode::UncheckedUnicode;

#[derive(Clone)]
pub struct VAscii {
    /// Minimum number of characters required (inclusive).
    min_length: u64,
    /// Maximum number of characters allowed (inclusive).
    max_length: u64,
    /// Minimum ascii code (inclusive)
    min_code: u8,
    /// Maximum ascii code (inclusive)
    max_code: u8,
}

impl VAscii {
    pub fn try_new(
        min_length: u64,
        max_length: u64,
        min_code: u8,
        max_code: u8,
    ) -> Result<VAscii, LqError> {
        if min_code > max_code {
            LqError::err_new(format!(
                "Min ascii code ({:?}) is greater than max ascii code ({:?}).",
                min_code, max_code
            ))
        } else if min_length > max_length {
            LqError::err_new(format!(
                "Min length of ascii characters ({:?}) is greater than maximum ({:?}).",
                min_code, max_code
            ))
        } else if min_code > 127 || max_code > 127 {
            LqError::err_new(format!(
                "Min ascii code / max ascii code must be within the ascii range (0-127; \
                 inclusive); have {:?} - {:?}.",
                min_code, max_code
            ))
        } else {
            Result::Ok(Self {
                min_length,
                max_length,
                min_code,
                max_code,
            })
        }
    }
}

impl<'a> From<VAscii> for Validators<'a> {
    fn from(value: VAscii) -> Self {
        Validators::Ascii(value)
    }
}

impl<'a> Validator<'a> for VAscii {

    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let bytes = UncheckedUnicode::de_serialize(context.reader())?;

        // first check length (that's faster)
        let length = bytes.len();
        if length < self.min_length as usize {
            return LqError::err_new(format!(
                "Given ascii is too small - not enough characters (minimum \
                 allowed is {:?})",
                self.min_length
            ));
        }
        if length as u64 > self.max_length {
            return LqError::err_new(format!(
                "Given ascii is too large - too many characters (maximum \
                 allowed is {:?})",
                self.max_length
            ));
        }

        // now we have to check each character
        for byte in bytes {
            let owned_byte = *byte;
            if owned_byte < self.min_code || owned_byte > self.max_code {
                return LqError::err_new(format!(
                    "Found a ascii character (code {:?}) that's not allowed. Allowed range is \
                     {:?} to {:?} (both inclusive)",
                    byte, self.min_code, self.max_code
                ));
            }
        }

        Result::Ok(())
    }
}
