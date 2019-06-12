use crate::converter::Converter;
use crate::core::Context;
use crate::core::Parser;
use crate::types::identifier::PIdentifier;
use crate::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::types::seq::TSeq;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::types::seq::SeqHeader;
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
        // Maybe it's a special case (seq -> ascii)
        let parsed_as_identifier = PIdentifier::maybe_parse(context, writer, value, r#type)?;
        if parsed_as_identifier {
            return Ok(());
        }

        // TODO: Since we now have a dedicated range type, automatically sort here?

        let seq = C::TConverter::require_seq(value.as_ref())?;
        let len = seq.len();
        let u32_len = u32::try_from(len)?;
        SeqHeader::serialize(writer, &SeqHeader::new(u32_len))?;
        for item in seq {
            context.parse(writer, r#type.element(), item)?;
        }
        Ok(())
    }
}
