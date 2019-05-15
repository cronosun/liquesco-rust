use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use liquesco_core::schema::seq::{TSeq};
use liquesco_core::serialization::core::Serializer;
use liquesco_core::serialization::seq::SeqHeader;
use std::convert::TryFrom;

pub struct PSeq;

impl Parser<'static> for PSeq {
    type T = TSeq;

    fn parse<'c, C>(
        context: &C,
        writer: &mut C::TWriter,
        r#type: &Self::T,
    ) -> Result<(), ParseError>
    where
        C: Context<'c>,
    {
        C::TConverter::require_no_name(context.text_value())?;

        let seq = C::TConverter::require_seq(context.value())?;
        let len = seq.len();
        let u32_len = u32::try_from(len)?;
        SeqHeader::serialize(writer, &SeqHeader::new(u32_len))?;
        for item in seq {
            context.parse(writer, r#type.element, item)?;
        }
        Ok(())
    }
}
