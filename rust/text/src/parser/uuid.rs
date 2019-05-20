use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::ParseError;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_schema::uuid::TUuid;
use liquesco_core::serialization::core::Serializer;

pub struct PUuid;

impl Parser<'static> for PUuid {
    type T = TUuid;

    fn parse<'c, C>(
        _: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        _: &Self::T,
    ) -> Result<(), ParseError>
    where
        C: Context<'c>,
    {
        C::TConverter::require_no_name(value)?;
        let text = C::TConverter::require_text(&value.value)?;
        let uuid = uuid::Uuid::parse_str(text);
        match uuid {
            Result::Ok(uuid) => {
                liquesco_core::serialization::uuid::Uuid::serialize(
                    writer,
                    &liquesco_core::serialization::uuid::Uuid::from(uuid.as_bytes()),
                )?;
                Ok(())
            }
            Result::Err(err) => Err(ParseError::new(format!(
                "Unable to parse given UUID string: {:?}; error {:?}",
                uuid, err
            ))),
        }
    }
}
