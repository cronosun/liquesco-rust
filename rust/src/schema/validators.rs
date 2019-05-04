use crate::common::error::LqError;
use crate::schema::core::DeSerializationContext;
use crate::schema::core::Schema;
use crate::schema::core::Validator;
use crate::schema::vstruct::VStruct;
use crate::schema::vuint::VUInt;
use crate::serialization::core::BinaryReader;
use crate::serialization::core::DeSerializer;
use crate::serialization::tenum::EnumHeader;

const ENUM_STRUCT: usize = 0;
const ENUM_UINT: usize = 1;

pub enum Validators<'a> {
    Struct(VStruct<'a>),
    UInt(VUInt),
}

impl<'a> Validators<'a> {
    pub fn de_serialize<T: DeSerializationContext<'a>>(
        context: &mut T,
    ) -> Result<Validators<'a>, LqError> {
        let enum_header = EnumHeader::de_serialize(context.reader())?;
        let ordinal = enum_header.ordinal();
        if !enum_header.has_value() {
            return LqError::err_new(format!(
                "Expecting an enum with a value; got no value; \
                 ordinal {:?}",
                ordinal
            ));
        }
        match ordinal {
            ENUM_STRUCT => Result::Ok(Validators::Struct(VStruct::de_serialize(context)?)),
            ENUM_UINT => Result::Ok(Validators::UInt(VUInt::de_serialize(context)?)),
            _ => LqError::err_new(format!(
                "Unknown validator type. Enum ordinal \
                 is {:?}",
                ordinal
            )),
        }
    }

    pub fn validate<S: Schema<'a>, R : BinaryReader<'a>>(&self, schema: &S, reader : &mut R) -> Result<(), LqError> {
        match self {
            Validators::Struct(value) => value.validate(schema, reader),
            Validators::UInt(value) => value.validate(schema, reader),
        }
    }
}

