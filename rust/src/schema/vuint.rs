use crate::common::error::LqError;
use crate::schema::core::Config;
use crate::schema::core::Validator;
use crate::serialization::core::BinaryReader;
use crate::serialization::core::DeSerializer;
use crate::serialization::tlist::ListHeader;
use crate::serialization::tuint::TUInt;

pub struct VUInt {
    min_value: u64,
    max_value: u64,
}

impl VUInt {
    pub fn try_new(min_value : u64, max_value : u64) -> Result<VUInt, LqError> {
        if min_value>max_value {
            LqError::err_new(format!("Min value ({:?}) is greater then max value ({:?}).",
            min_value, max_value))
        } else {
            Result::Ok(Self {
                min_value,
                max_value
            })
        }
    }
}

impl<'a> Validator<'a> for VUInt {
    fn validate<T: BinaryReader<'a>>(&self, reader: &mut T, _: &Config) -> Result<(), LqError> {
        let int_value = TUInt::de_serialize(reader)?;
        if int_value < self.min_value {
            return LqError::err_new(format!("Given integer {:?} is too small (minimum \
            allowed is {:?})", int_value, self.min_value));
        }
        if int_value > self.max_value {
            return LqError::err_new(format!("Given integer {:?} is too large (maximum \
            allowed is {:?})", int_value, self.max_value));
        }
        Result::Ok(())
    }
}

impl<'a> DeSerializer<'a> for VUInt {
    type Item = Self;

    fn de_serialize<T: BinaryReader<'a>>(reader: &mut T) -> Result<Self::Item, LqError> {
        let header = ListHeader::de_serialize(reader)?;
        header.read_struct(reader, 2, |reader| {
            Self::Item::try_new(
                TUInt::de_serialize(reader)?,
                TUInt::de_serialize(reader)?)
        })
    }
}