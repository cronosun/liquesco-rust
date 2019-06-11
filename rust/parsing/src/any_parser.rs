use crate::types::ascii::PAscii;
use crate::types::binary::PBinary;
use crate::types::boolean::PBool;
use crate::types::enumeration::PEnum;
use crate::types::float::PFloat32;
use crate::types::float::PFloat64;
use crate::types::key_ref::PKeyRef;
use crate::types::map::PMap;
use crate::types::option::POption;
use crate::types::range::PRange;
use crate::types::root_map::PRootMap;
use crate::types::seq::PSeq;
use crate::types::sint::PSInt;
use crate::types::structure::PStruct;
use crate::types::uint::PUInt;
use crate::types::unicode::PUnicode;
use crate::types::uuid::PUuid;
use crate::value::TextValue;
use crate::core::Context;
use crate::core::Parser;
use liquesco_common::error::LqError;
use liquesco_schema::any_type::AnyType;

pub(crate) fn parse_any<'c, C>(
    context: &mut C,
    any_type: &AnyType,
    text_value: &TextValue,
    writer: &mut C::TWriter,
) -> Result<(), LqError>
where
    C: Context<'c>,
{
    match any_type {
        AnyType::Option(value) => POption::parse(context, writer, text_value, value),
        AnyType::UInt(value) => PUInt::parse(context, writer, text_value, value),
        AnyType::SInt(value) => PSInt::parse(context, writer, text_value, value),
        AnyType::Struct(value) => PStruct::parse(context, writer, text_value, value),
        AnyType::Seq(value) => PSeq::parse(context, writer, text_value, value),
        AnyType::Binary(value) => PBinary::parse(context, writer, text_value, value),
        AnyType::Ascii(value) => PAscii::parse(context, writer, text_value, value),
        AnyType::Enum(value) => PEnum::parse(context, writer, text_value, value),
        AnyType::Bool(value) => PBool::parse(context, writer, text_value, value),
        AnyType::Unicode(value) => PUnicode::parse(context, writer, text_value, value),
        AnyType::Float32(value) => PFloat32::parse(context, writer, text_value, value),
        AnyType::Float64(value) => PFloat64::parse(context, writer, text_value, value),
        AnyType::Uuid(value) => PUuid::parse(context, writer, text_value, value),
        AnyType::Range(value) => PRange::parse(context, writer, text_value, value),
        AnyType::Map(value) => PMap::parse(context, writer, text_value, value),
        AnyType::RootMap(value) => PRootMap::parse(context, writer, text_value, value),
        AnyType::KeyRef(value) => PKeyRef::parse(context, writer, text_value, value),
    }
}
