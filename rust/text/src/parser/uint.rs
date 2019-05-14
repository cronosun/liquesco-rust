use liquesco_core::schema::uint::TUInt;
use liquesco_core::serialization::core::Serializer;
use liquesco_core::serialization::uint::UInt64;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::value::Converter;

pub struct PUInt;

impl Parser<'static> for PUInt {
    type T = TUInt;

    fn parse<'c, C>(context: &C, writer : &mut C::TWriter, _: &Self::T) -> Result<(), ParseError>
        where
            C: Context<'c> {
        C::TConverter::require_no_name(context.text_value())?;
        let value = C::TConverter::require_u64(context.value())?;
        UInt64::serialize(writer, &value)?;
        Result::Ok(())
    }
}
