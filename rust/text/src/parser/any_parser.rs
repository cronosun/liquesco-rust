use crate::parser::value::TextValue;
use crate::parser::ascii::PAscii;
use crate::parser::boolean::PBool;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::enumeration::PEnum;
use crate::parser::option::POption;
use crate::parser::seq::PSeq;
use crate::parser::sint::PSInt;
use crate::parser::structure::PStruct;
use crate::parser::uint::PUInt;
use crate::parser::unicode::PUnicode;
use crate::parser::float::PFloat32;
use crate::parser::float::PFloat64;
use liquesco_core::schema::any_type::AnyType;

pub(crate) fn parse_any<'c, C>(
    context: &mut C,
    any_type: &AnyType,
    text_value: &TextValue,
    writer: &mut C::TWriter,
) -> Result<(), ParseError>
where
    C: Context<'c>,
{
    match any_type {
        AnyType::Option(value) => POption::parse(context, writer, text_value, value),
        AnyType::UInt(value) => PUInt::parse(context, writer, text_value, value),
        AnyType::SInt(value) => PSInt::parse(context, writer,text_value, value),
        AnyType::Struct(value) => PStruct::parse(context, writer,text_value, value),
        AnyType::Seq(value) => PSeq::parse(context, writer,text_value, value),
        AnyType::Ascii(value) => PAscii::parse(context, writer, text_value,value),
        AnyType::Enum(value) => PEnum::parse(context, writer, text_value,value),
        AnyType::Bool(value) => PBool::parse(context, writer,text_value, value),
        AnyType::Unicode(value) => PUnicode::parse(context, writer,text_value, value),
        AnyType::Float32(value) => PFloat32::parse(context, writer, text_value,value),
        AnyType::Float64(value) => PFloat64::parse(context, writer,text_value, value),
        _ => Result::Err(ParseError::new(format!(
            "No parser for type {:?} implemented",
            any_type
        ))),
    }
}
