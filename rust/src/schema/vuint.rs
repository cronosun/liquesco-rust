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

pub struct VUInt {
    min_value: u64,
    max_value: u64,
}

impl VUInt {
    pub fn try_new(min_value: u64, max_value: u64) -> Result<Self, LqError> {
        if min_value > max_value {
            LqError::err_new(format!(
                "Min value ({:?}) is greater then max value ({:?}).",
                min_value, max_value
            ))
        } else {
            Result::Ok(Self {
                min_value,
                max_value,
            })
        }
    }
}

impl<'a> From<VUInt> for Validators<'a> {
    fn from(value: VUInt) -> Self {
        Validators::UInt(value)
    }
}

impl<'a> Validator<'a> for VUInt {
    type DeSerItem = Self;

    fn validate<S, R>(&self, _: &S, reader: &mut R) -> Result<(), LqError>
    where
        S: Schema<'a>,
        R: BinaryReader<'a>,
    {
        let int_value = UInt64::de_serialize(reader)?;
        if int_value < self.min_value {
            return LqError::err_new(format!(
                "Given integer {:?} is too small (minimum \
                 allowed is {:?})",
                int_value, self.min_value
            ));
        }
        if int_value > self.max_value {
            return LqError::err_new(format!(
                "Given integer {:?} is too large (maximum \
                 allowed is {:?})",
                int_value, self.max_value
            ));
        }
        Result::Ok(())
    }

    fn de_serialize<TContext>(context: &mut TContext) -> Result<Self::DeSerItem, LqError>
    where
        TContext: DeSerializationContext<'a>,
    {
        let header = SeqHeader::de_serialize(context.reader())?;
        header.read_struct(context.reader(), 2, |reader| {
            Self::DeSerItem::try_new(UInt64::de_serialize(reader)?, UInt64::de_serialize(reader)?)
        })
    }

    fn serialize<S, W>(&self, _: &S, writer: &mut W) -> Result<(), LqError>
    where
        S: Schema<'a>,
        W: BinaryWriter,
    {
        let header = SeqHeader::new(2);
        SeqHeader::serialize(writer, &header)?;
        UInt64::serialize(writer, &self.min_value)?;
        UInt64::serialize(writer, &self.max_value)
    }
}
