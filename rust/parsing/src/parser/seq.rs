use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::seq::TSeq;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::seq::SeqHeader;
use std::convert::TryFrom;

pub struct PSeq;

impl<'a> Parser<'a> for PSeq {
    type T = TSeq<'a>;

    fn parse<'c, C>(
        context: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        r#type: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        C::TConverter::require_no_name(value)?;

        let seq = C::TConverter::require_seq(value.as_ref())?;
        let len = seq.len();
        let u32_len = u32::try_from(len)?;
        SeqHeader::serialize(writer, &SeqHeader::new(u32_len))?;
        for item in seq {
            context.parse(writer, r#type.element, item)?;
        }
        Ok(())
    }
}
