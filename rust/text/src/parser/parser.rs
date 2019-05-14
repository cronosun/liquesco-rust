use crate::parser::any_parser::parse_any;
use crate::parser::core::{Context, ParseError};
use crate::parser::value::{Converter, TextValue, Value};
use liquesco_core::schema::core::{Schema, TypeRef};
use liquesco_core::serialization::vec_writer::VecWriter;
use std::marker::PhantomData;

pub(crate) struct ParserContext<'se, 's, 'v, TSchema>
where
    TSchema: Schema<'s>,
{
    pub(crate) value: &'se TextValue<'v>,
    pub(crate) schema: &'se TSchema,
    pub(crate) _phantom : &'s PhantomData<()>
}

pub struct DefaultConverter;

impl Converter for DefaultConverter {}

impl<'se, 's, 'v, STSchema> Context<'s> for ParserContext<'se, 's, 'v, STSchema>
where
    STSchema: Schema<'s>,
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
            _phantom : &PhantomData
        };

        // TODO: Add position if position is missing
        parse_any(&context, any_type, writer)
    }
}
