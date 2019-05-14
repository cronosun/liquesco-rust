use crate::core::ParseError;
use crate::core::Context;
use liquesco_core::schema::any_type::AnyType;
use crate::uint::PUInt;
use crate::structure::PStruct;
use crate::option::POption;
use crate::core::Parser;

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
        _ => {
            Result::Err(ParseError::new("No parser for type implemented"))
        }
    }
}