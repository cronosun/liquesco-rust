use crate::common::error::LqError;
use crate::schema::core::{Config, Validator};
use crate::schema::vstruct::VStruct;
use crate::schema::vuint::VUInt;
use crate::serialization::core::{BinaryReader, DeSerializer};
use crate::serialization::tenum::EnumHeader;

const ENUM_STRUCT: usize = 0;
const ENUM_UINT: usize = 1;

pub enum Validators<'a> {
    Struct(VStruct<'a>),
    UInt(VUInt),
}

impl<'a> Validator<'a> for Validators<'a> {
    fn validate<T: BinaryReader<'a>>(&self, reader: &mut T, config: &Config)-> Result<(), LqError> {
        match self {
            Validators::Struct(value) => value.validate(reader, config),
            Validators::UInt(value) => value.validate(reader, config),
        }
    }
}

impl<'a> DeSerializer<'a> for Validators<'a> {
    type Item = Self;

    fn de_serialize<T: BinaryReader<'a>>(reader: &mut T) -> Result<Self::Item, LqError> {
        let enum_header = EnumHeader::de_serialize(reader)?;
        let ordinal = enum_header.ordinal();
        if !enum_header.has_value() {
            return LqError::err_new(format!("Expecting an enum with a value; got no value; \
            ordinal {:?}", ordinal));
        }
        match ordinal {
            ENUM_STRUCT => {
                Result::Ok(Validators::Struct(VStruct::de_serialize(reader)?))
            }
            ENUM_UINT => {
                Result::Ok(Validators::UInt(VUInt::de_serialize(reader)?))
            }
            _ => {
                LqError::err_new(format!("Unknown validator type. Enum ordinal \
                is {:?}", ordinal))
            }
        }
    }
}