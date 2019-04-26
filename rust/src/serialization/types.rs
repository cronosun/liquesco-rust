use crate::serialization::core::BinaryReader;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::LqError;
use crate::serialization::core::SkipMore;
use crate::serialization::core::TypeId;
use crate::serialization::tutf8::TUtf8;
use crate::serialization::toption::Presence;
use crate::serialization::tbinary::TBinary;
use crate::serialization::tstruct::StructInfo;
use crate::serialization::type_ids::BLOCK_ID_UTF8;
use crate::serialization::type_ids::BLOCK_ID_BINARY;
use crate::serialization::type_ids::BLOCK_ID_BASE0;
use crate::serialization::type_ids::BLOCK_ID_BASE1;
use crate::serialization::type_ids::BLOCK_ID_BASE2;
use crate::serialization::type_ids::BLOCK_ID_BASE3;
use crate::serialization::type_ids::TYPE_BOOL_FALSE;
use crate::serialization::type_ids::TYPE_BOOL_TRUE;
use crate::serialization::type_ids::TYPE_OPTION_ABSENT;
use crate::serialization::type_ids::TYPE_OPTION_PRESENT;
use crate::serialization::type_ids::TYPE_STRUCT_0;
use crate::serialization::type_ids::TYPE_STRUCT_U16;

pub enum Types {
    TBool,
    TUtf8,
    TBinary,
    TOption,
    TStruct,
}

impl Types {
    pub fn skip<'a, T: BinaryReader<'a>>(&self, reader: &mut T) -> Result<SkipMore, LqError> {
        match self {
            Types::TUtf8 => TUtf8::skip(reader),
            Types::TBinary => TBinary::skip(reader),
            Types::TBool => bool::skip(reader),
            Types::TOption => Presence::skip(reader),
            Types::TStruct => StructInfo::skip(reader),
        }
    }

    pub fn from_id(type_id: TypeId) -> Result<Types, LqError> {
        let block_id = type_id.block();
        match block_id {
            BLOCK_ID_BASE0 | BLOCK_ID_BASE1 | BLOCK_ID_BASE2 | BLOCK_ID_BASE3 => {
                match type_id {
                        TYPE_BOOL_FALSE | TYPE_BOOL_TRUE => Result::Ok(Types::TBool),
                        TYPE_OPTION_ABSENT | TYPE_OPTION_PRESENT => Result::Ok(Types::TOption),
                        n if n >= TYPE_STRUCT_0 && n <= TYPE_STRUCT_U16 => Result::Ok(Types::TStruct),
                        _ => LqError::err_new(format!("Unknown type id: {:?}", type_id)),
                }
            },
            BLOCK_ID_UTF8 => Result::Ok(Types::TUtf8),
            BLOCK_ID_BINARY => Result::Ok(Types::TBinary),
            _ => LqError::err_new(format!("Unknown type id: {:?}", type_id)),
        }
    }
}
