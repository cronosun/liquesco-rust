use crate::parser::core::AnchorInfo;
use crate::parser::any_parser::parse_any;
use crate::parser::core::{Context, ParseError};
use crate::parser::value::{TextValue};
use crate::parser::converter::Converter;
use liquesco_core::schema::core::{Schema, TypeRef};
use liquesco_core::serialization::vec_writer::VecWriter;
use std::marker::PhantomData;

pub(crate) struct ParserContext<'se, 's, TSchema>
where
    TSchema: Schema<'s>,
{
    pub(crate) schema: &'se TSchema,
    pub(crate) anchor_info: Option<AnchorInfo>,
    pub(crate) _phantom : &'s PhantomData<()>
}

pub struct DefaultConverter;

impl Converter for DefaultConverter {}

impl<'se, 's, 'v, STSchema> Context<'s> for ParserContext<'se, 's, STSchema>
where
    STSchema: Schema<'s>,
{
    type TConverter = DefaultConverter;
    type TSchema = STSchema;
    type TWriter = VecWriter;

    fn schema(&self) -> &Self::TSchema {
        self.schema
    }

    fn parse(
        &mut self,
        writer: &mut Self::TWriter,
        r#type: TypeRef,
        value: &TextValue,
    ) -> Result<(), ParseError> {
        let maybe_any_type = self.schema().maybe_type(r#type);
        let any_type = maybe_any_type.unwrap(); // TODO

        let mut context = ParserContext {
            schema: self.schema,
            anchor_info : Option::None,
            _phantom : &PhantomData
        };

        // TODO: Add position if position is missing
        parse_any(&mut context, any_type, value, writer)
    }

    fn anchor_info(&mut self) -> &mut Option<AnchorInfo> {
        &mut self.anchor_info
    }
}
