use crate::parser::any_parser::parse_any;
use crate::parser::converter::Converter;
use crate::parser::core::AnchorInfo;
use crate::parser::core::Context;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::core::{Schema, TypeRef};
use liquesco_serialization::vec_writer::VecWriter;
use std::marker::PhantomData;
use liquesco_serialization::core::ToVecLqWriter;

pub(crate) struct ParserContext<'se, 's, TSchema>
where
    TSchema: Schema,
{
    pub(crate) schema: &'se TSchema,
    pub(crate) anchor_info: Option<Vec<AnchorInfo>>,
    pub(crate) _phantom: &'s PhantomData<()>,
}

pub struct DefaultConverter;

impl Converter for DefaultConverter {}

impl<'se, 's, 'v, STSchema> Context<'s> for ParserContext<'se, 's, STSchema>
where
    STSchema: Schema,
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
        r#type: &TypeRef,
        value: &TextValue,
    ) -> Result<(), LqError> {
        let taken_anchor_info = self.anchor_info.take();

        let maybe_any_type = self.schema().maybe_type(r#type);
        let any_type = maybe_any_type.unwrap(); // TODO

        let mut context = ParserContext {
            schema: self.schema,
            anchor_info: taken_anchor_info,
            _phantom: &PhantomData,
        };

        // TODO: Add position if position is missing
        let result = parse_any(&mut context, any_type, value, writer);

        self.anchor_info = context.anchor_info;

        result
    }

    fn parse_to_vec(
        &mut self,
         r#type: &TypeRef,
        value: &TextValue) -> Result<Vec<u8>, LqError> {
            let mut vec_writer = VecWriter::default();
            self.parse(&mut vec_writer, r#type, value)?;
            Ok(vec_writer.into_vec())
        }
}
