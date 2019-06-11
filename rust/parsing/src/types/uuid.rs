use crate::converter::Converter;
use crate::core::Context;
use crate::core::Parser;
use crate::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::types::uuid::TUuid;
use liquesco_serialization::core::Serializer;

pub struct PUuid;

impl<'a> Parser<'a> for PUuid {
    type T = TUuid<'a>;

    fn parse<'c, C>(
        _: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        _: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let text = C::TConverter::require_text(&value.value)?;
        let uuid = uuid::Uuid::parse_str(text);
        match uuid {
            Result::Ok(uuid) => {
                liquesco_serialization::types::uuid::Uuid::serialize(
                    writer,
                    &liquesco_serialization::types::uuid::Uuid::from(uuid.as_bytes()),
                )?;
                Ok(())
            }
            Result::Err(err) => Err(LqError::new(format!(
                "Unable to parse given UUID string: {:?}; error {:?}",
                uuid, err
            ))),
        }
    }
}
