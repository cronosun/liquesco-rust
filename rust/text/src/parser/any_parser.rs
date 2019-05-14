use crate::parser::core::ParseError;
use crate::parser::core::Context;
use liquesco_core::schema::any_type::AnyType;
use crate::parser::uint::PUInt;
use crate::parser::seq::PSeq;
use crate::parser::structure::PStruct;
use crate::parser::enumeration::PEnum;
use crate::parser::option::POption;
use crate::parser::core::Parser;
use crate::parser::ascii::PAscii;

pub(crate) fn parse_any<'c, C>(context : &C, any_type : &AnyType, writer : &mut C::TWriter) -> Result<(), ParseError> where C : Context<'c> {
    match any_type {
        AnyType::Option(value) => {
            POption::parse(context, writer, value)
        },
        AnyType::UInt(value) => {
            PUInt::parse(context, writer, value)
        },
        AnyType::Struct(value) => {
            PStruct::parse(context, writer, value)
        },
        AnyType::Seq(value) => {
            PSeq::parse(context, writer, value)
        },
        AnyType::Ascii(value) => {
            PAscii::parse(context, writer, value)
        }
        AnyType::Enum(value) => {
            PEnum::parse(context, writer, value)
        }
        _ => {
            Result::Err(ParseError::new(format!("No parser for type {:?} implemented", any_type)))
        }
    }
}