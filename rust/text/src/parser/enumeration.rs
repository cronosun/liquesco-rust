use crate::parser::converter::Converter;
use crate::parser::converter::IdentifierType;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::value::Value;
use liquesco_core::schema::enumeration::TEnum;
use liquesco_core::serialization::core::Serializer;
use liquesco_core::serialization::seq::SeqHeader;
use std::convert::TryFrom;

pub struct PEnum;

impl<'a> Parser<'a> for PEnum {
    type T = TEnum<'a>;

    fn parse<'c, C>(
        context: &C,
        writer: &mut C::TWriter,
        r#type: &Self::T,
    ) -> Result<(), ParseError>
    where
        C: Context<'c>,
    {
        C::TConverter::require_no_name(context.text_value())?;

        // enum cam be either a string or a sequence (containting a string and 1-n values)
        let cloned_value = context.value().clone();
        let enum_name = match &cloned_value {
            Value::Text(value) => value,
            Value::Seq(seq) => {
                if seq.len() < 1 {
                    return Err(ParseError::new(
                        format!("Expecting the value for an enum. Value for \
                enum is either a single string (tag) or a sequence starting with a string \
                (tag) and 1-n values; got a sequence but does not start with a string. Got: {:?}", 
                context.text_value()),
                    ));
                }
                C::TConverter::require_text(&seq[0].value)?
            }
            _ => {
                return Err(ParseError::new(format!(
                    "Expecting the value for an enum. Value for \
                     enum is either a single string (tag) or a sequence starting with a string \
                     (tag) and 1-n values. Got: {:?}",
                    context.text_value()
                )));
            }
        };
        let enum_identifier =
            C::TConverter::string_to_identifier(enum_name, IdentifierType::EnumIdentifier)?;

        // find the correct variant
        let maybe_variant = r#type
            .0
            .iter()
            .find(|item| item.identifier() == &enum_identifier);

        if let Some(variant) = maybe_variant {
            Ok(())
        } else {
            Err(ParseError::new(format!(
                "There's no enum variant called {:?}.",
                enum_name
            )))
        }
    }
}
