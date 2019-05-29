use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::value::Text;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::ascii::TAscii;
use liquesco_schema::core::TypeContainer;
use liquesco_schema::seq::TSeq;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::seq::SeqHeader;
use liquesco_serialization::unicode::Unicode;
use std::convert::TryFrom;

pub struct PIdentifier;

impl PIdentifier {
    /// Special parsing for identifiers / sequences
    pub fn maybe_parse<'a, 'c, C>(
        context: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        r#type: &TSeq<'a>,
    ) -> Result<bool, LqError>
    where
        C: Context<'c>,
    {
        // must be a sequence that contains ascii
        let element_ref = r#type.element();
        if let Some(element) = context.schema().maybe_type(element_ref) {
            match element {
                AnyType::Ascii(ascii) => {
                    if let Some(text) = C::TConverter::to_text(&value.value) {
                        Self::try_to_parse::<C>(writer, ascii, text)?;
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    fn try_to_parse<'c, C>(
        writer: &mut C::TWriter,
        ascii: &TAscii,
        text: &Text,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let mut segments: Vec<String> = vec![];
        let mut current = String::new();
        for chr in text.chars() {
            if ascii.codes().contains_chr(chr) {
                current.push(chr);
            } else {
                // outside range
                segments.push(current);
                current = String::new();
            }
        }
        // push last
        segments.push(current);

        // now push all those strings
        let len = segments.len();
        let u32_len = u32::try_from(len)?;
        SeqHeader::serialize(writer, &SeqHeader::new(u32_len))?;
        for segment in segments {
            Unicode::serialize(writer, &segment)?;
        }
        Ok(())
    }
}
