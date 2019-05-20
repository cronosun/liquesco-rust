use crate::parser::converter::Converter;
use crate::parser::converter::IdentifierType;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use crate::parser::value::Value;
use liquesco_schema::structure::TStruct;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::seq::SeqHeader;
use std::convert::TryFrom;

pub struct PStruct;

impl<'a> Parser<'a> for PStruct {
    type T = TStruct<'a>;

    fn parse<'c, C>(
        context: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        r#type: &Self::T,
    ) -> Result<(), ParseError>
    where
        C: Context<'c>,
    {
        C::TConverter::require_no_name(value)?;

        // for structures the input must be a map
        let mut value = C::TConverter::require_string_map(value.as_ref())?;

        let number_of_fields = r#type.fields().len();
        let u32_number_of_fields = u32::try_from(number_of_fields)?;
        SeqHeader::serialize(writer, &SeqHeader::new(u32_number_of_fields))?;

        for field in r#type.fields() {
            let identifier = &field.identifier;
            let r#type = field.r#type;
            let identifier_string =
                C::TConverter::identifier_to_string(identifier, IdentifierType::StructField);

            let key: &str = &identifier_string;
            match value.remove(key) {
                Option::Some(value) => {
                    context.parse(writer, r#type, value)?;
                }
                // we also accept no value (in this case it's no value)
                Option::None => {
                    let text_value: TextValue<'static> = Value::Nothing.into();
                    context.parse(writer, r#type, &text_value)?;
                }
            };
        }

        // the map should now be empty (all fields processed)
        if value.len() > 0 {
            Result::Err(ParseError::new(format!(
                "Not all fields have been processed (consumed). \
                 There are unprocessed field(s): {:?}. Value: {:?}; Type: {:?}",
                value.keys(),
                value,
                r#type
            )))
        } else {
            Result::Ok(())
        }
    }
}
