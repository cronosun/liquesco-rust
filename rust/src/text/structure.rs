use crate::schema::structure::TStruct;
use crate::text::core::Context;
use crate::text::core::ParseError;
use crate::text::core::Parser;
use crate::text::value::Converter;
use crate::text::value::IdentifierType;
use crate::text::value::TextValue;
use crate::text::value::Value;

pub struct PStruct;

impl<'a> Parser<'a> for PStruct {
    type T = TStruct<'a>;

    fn parse<C>(context: &mut C, writer: &mut C::TWriter, r#type: Self::T) -> Result<(), ParseError>
    where
        C: Context,
    {
        C::TConverter::require_no_name(context.text_value())?;

        // for structures the input must be a map
        let mut value = C::TConverter::require_string_map(context.value())?;

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
                    let text_value: TextValue<'static> = Value::Maybe(Option::None).into();
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
                context.text_value(),
                r#type
            )))
        } else {
            Result::Ok(())
        }
    }
}
