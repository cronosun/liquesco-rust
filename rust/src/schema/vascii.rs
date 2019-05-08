use crate::common::error::LqError;
use crate::schema::core::DeSerializationContext;
use crate::schema::core::Schema;
use crate::schema::core::Validator;
use crate::schema::validators::Validators;
use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::Serializer;
use crate::serialization::tseq::SeqHeader;
use crate::serialization::tuint::UInt64;
use crate::serialization::tuint::UInt8;
use crate::serialization::tunicode::UncheckedUnicode;

pub struct VAscii {
    /// Minimum number of characters required (inclusive).
    min_characters: u64,
    /// Maximum number of characters allowed (inclusive).
    max_characters: u64,
    /// Minimum ascii code (inclusive)
    min_value: u8,
    /// Maximum ascii code (inclusive)
    max_value: u8,
}

impl VAscii {
    pub fn try_new(
        min_characters: u64,
        max_characters: u64,
        min_value: u8,
        max_value: u8,
    ) -> Result<VAscii, LqError> {
        if min_value > max_value {
            LqError::err_new(format!(
                "Min value ({:?}) is greater then max value ({:?}).",
                min_value, max_value
            ))
        } else if min_characters > max_characters {
            LqError::err_new(format!(
                "Min characters ({:?}) is greater then max charactewrs ({:?}).",
                min_value, max_value
            ))
        } else if min_value > 127 || max_value > 127 {
            LqError::err_new(format!(
                "Min value / max value must be within the ascii rang (0-127; inclusive); have {:?} - {:?}.",
                min_value, max_value
            ))
        } else {
            Result::Ok(Self {
                min_characters,
                max_characters,
                min_value,
                max_value,
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
    type DeSerItem = Self;

    fn validate<S, R>(&self, _: &S, reader: &mut R) -> Result<(), LqError>
    where
        S: Schema<'a>,
        R: BinaryReader<'a>,
    {
        let bytes = UncheckedUnicode::de_serialize(reader)?;

        // first check length (that's faster)
        let length = bytes.len();
        if length < self.min_characters as usize {
            return LqError::err_new(format!(
                "Given ascii is too small - not enough characters (minimum \
                 allowed is {:?})",
                self.min_characters
            ));
        }
        if length as u64 > self.max_characters {
            return LqError::err_new(format!(
                "Given ascii is too large - too many characters (maximum \
                 allowed is {:?})",
                self.max_characters
            ));
        }

        // now we have to check each character
        for byte in bytes {
            let owned_byte = *byte;
            if owned_byte < self.min_value || owned_byte > self.max_value {
                return LqError::err_new(format!(
                "Found a ascii character (code {:?}) that's not allowed. Allowed range is {:?} to {:?} (both inclusive)",
                byte, self.min_value, self.max_value
            ));
            }
        }

        Result::Ok(())
    }

    fn de_serialize<TContext>(context: &mut TContext) -> Result<Self::DeSerItem, LqError>
    where
        TContext: DeSerializationContext<'a>,
    {
        let header = SeqHeader::de_serialize(context.reader())?;
        header.read_struct(context.reader(), 4, |reader| {
            Self::DeSerItem::try_new(
                UInt64::de_serialize(reader)?,
                UInt64::de_serialize(reader)?,
                UInt8::de_serialize(reader)?,
                UInt8::de_serialize(reader)?,
            )
        })
    }

    fn serialize<S, W>(&self, _: &S, writer: &mut W) -> Result<(), LqError>
    where
        S: Schema<'a>,
        W: BinaryWriter,
    {
        let header = SeqHeader::new(4);
        SeqHeader::serialize(writer, &header)?;
        UInt64::serialize(writer, &self.min_characters)?;
        UInt64::serialize(writer, &self.max_characters)?;
        UInt8::serialize(writer, &self.min_value)?;
        UInt8::serialize(writer, &self.max_value)
    }
}
