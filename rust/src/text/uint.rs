use crate::schema::uint::TUInt;
use crate::serialization::core::Serializer;
use crate::serialization::uint::UInt64;
use crate::text::core::Context;
use crate::text::core::ParseError;
use crate::text::core::Parser;
use crate::text::value::Converter;

pub struct PUInt;

impl Parser<'static> for PUInt {
    type T = TUInt;

    fn parse<C>(context: &mut C, writer : &mut C::TWriter, _: Self::T) -> Result<(), ParseError>
    where
        C: Context
    {
        C::TConverter::require_no_name(context.text_value())?;
        let value = C::TConverter::require_u64(context.value())?;
        UInt64::serialize(writer, &value)?;
        Result::Ok(())
    }
}
