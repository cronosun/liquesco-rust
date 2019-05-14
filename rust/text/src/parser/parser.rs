use crate::any_parser::parse_any;
use crate::core::{Context, ParseError};
use crate::value::{Converter, TextValue, Value};
use liquesco_core::schema::core::{Schema, TypeRef};
use liquesco_core::serialization::vec_writer::VecWriter;

pub struct ParserContext<'a, 'b, TSchema>
where
    TSchema: Schema<'a>,
{
    value: &'b TextValue<'b>,
    schema: &'a TSchema,
}

pub struct DefaultConverter;

impl Converter for DefaultConverter {}

impl<'a, 'b, STSchema> Context<'a> for ParserContext<'a, 'b, STSchema>
where
    STSchema: Schema<'a>,
{
    type TConverter = DefaultConverter;
    type TSchema = STSchema;
    type TWriter = VecWriter;

    fn schema(&self) -> &Self::TSchema {
        self.schema
    }

    fn value(&self) -> &Value {
        &self.value.value
    }

    fn text_value(&self) -> &TextValue {
        self.value
    }

    fn parse(
        &self,
        writer: &mut Self::TWriter,
        r#type: TypeRef,
        value: &TextValue,
    ) -> Result<(), ParseError> {
        let maybe_any_type = self.schema().maybe_type(r#type);
        let any_type = maybe_any_type.unwrap(); // TODO

        let context = ParserContext {
            value,
            schema: self.schema,
        };

        // TODO: Add position if position is missing
        parse_any(&context, any_type, writer)
    }
}
