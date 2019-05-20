use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_core::schema::reference::TReference;
use liquesco_core::serialization::core::Serializer;
use liquesco_core::serialization::uint::UInt32;

pub struct PReference;

impl Parser<'static> for PReference {
    type T = TReference;

    fn parse<'c, C>(
        context: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        _: &Self::T,
    ) -> Result<(), ParseError>
    where
        C: Context<'c>,
    {
        C::TConverter::require_no_name(value)?;
        let reference_as_text = C::TConverter::require_text(value.as_ref())?;
        C::TConverter::validate_reference(reference_as_text)?;
        let reference_as_u32 = context.present_anchor_info().reference(reference_as_text);
        UInt32::serialize(writer, &reference_as_u32)?;
        Result::Ok(())
    }
}
