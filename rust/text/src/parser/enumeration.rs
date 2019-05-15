use crate::parser::converter::Converter;
use crate::parser::converter::IdentifierType;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::value::Value;
use liquesco_core::schema::enumeration::TEnum;
use liquesco_core::schema::identifier::Identifier;
use liquesco_core::serialization::core::Serializer;
use liquesco_core::serialization::enumeration::EnumHeader;

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

        let maybe_variant_id = extract_variant_identifier::<C::TConverter>(context.value())?;
        if let Some(variant_id) = maybe_variant_id {
            let maybe_variant = r#type.variant_by_id(&variant_id);
            if let Some(variant_tuple) = maybe_variant {
                let ordinal = variant_tuple.0;
                let variant = variant_tuple.1;

                let number_of_expected_values = variant.values.len();
                let u32_number_of_expected_values = u32::try_from(number_of_expected_values)?;
                EnumHeader::serialize(
                    writer,
                    &EnumHeader::new(ordinal, u32_number_of_expected_values),
                )?;

                // now the values
                let number_of_values = match context.value() {
                    Value::Seq(seq) => seq.len() - 1,
                    _ => 0,
                };

                if number_of_values != number_of_expected_values {
                    return Err(ParseError::new(format!(
                        "The enum variant {:?} \
                         requires {:?} values. You specified {:?} values.",
                        variant_id, number_of_expected_values, number_of_values
                    )));
                }

                if let Value::Seq(seq) = context.value() {
                    for idx in 0..number_of_expected_values {
                        context.parse(writer, variant.values[idx], &seq[idx + 1])?;
                    }
                }

                Ok(())
            } else {
                Err(ParseError::new(format!(
                    "No such enum variant found: {:?}",
                    variant_id
                )))
            }
        } else {
            Err(ParseError::new(format!(
                "Could not extract enum variant identifier from \
                 given value. An enum variant is either just a string (variant tag; variants \
                 without values) or a sequence where the first element is a string (variant tag) \
                 and 1-n values. Got: {:?}",
                context.text_value()
            )))
        }
    }
}

/// enum can be either a string or a sequence (containting a string and 1-n values)
fn extract_variant_identifier<'a, 'v, T: Converter>(
    value: &'a Value<'v>,
) -> Result<Option<Identifier<'a>>, ParseError> {
    match value {
        Value::Text(text) => {
            let x: &'a str = &text;
            Ok(Option::Some(T::string_to_identifier(
                x,
                IdentifierType::EnumIdentifier,
            )?))
        }
        Value::Seq(seq) => match &seq[0].value {
            Value::Text(text) => {
                let x: &'a str = &text;
                Ok(Option::Some(T::string_to_identifier(
                    x,
                    IdentifierType::EnumIdentifier,
                )?))
            }
            _ => Ok(Option::None),
        },
        _ => Ok(Option::None),
    }
}