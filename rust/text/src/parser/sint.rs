use liquesco_core::schema::sint::TSInt;
use liquesco_core::serialization::core::Serializer;
use liquesco_core::serialization::sint::SInt64;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::converter::Converter;

pub struct PSInt;

impl Parser<'static> for PSInt {
    type T = TSInt;

    fn parse<'c, C>(context: &C, writer : &mut C::TWriter, _: &Self::T) -> Result<(), ParseError>
        where
            C: Context<'c> {
        C::TConverter::require_no_name(context.text_value())?;
        let value = C::TConverter::require_i64(context.value())?;
        SInt64::serialize(writer, &value)?;
        Result::Ok(())
    }
}
