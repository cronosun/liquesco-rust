use crate::parser::any_parser::parse_any;
use crate::parser::converter::Converter;
use crate::parser::core::AnchorInfo;
use crate::parser::core::{Context, ParseError};
use crate::parser::value::TextValue;
use liquesco_core::schema::core::{Schema, TypeRef};
use liquesco_core::serialization::vec_writer::VecWriter;
use std::marker::PhantomData;

pub(crate) struct ParserContext<'se, 's, TSchema>
where
    TSchema: Schema<'s>,
{
    pub(crate) schema: &'se TSchema,
    pub(crate) anchor_info: Option<AnchorInfo>,
    pub(crate) _phantom: &'s PhantomData<()>,
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
        let taken_anchor_info = self.take_anchor_info();
        let maybe_any_type = self.schema().maybe_type(r#type);
        let any_type = maybe_any_type.unwrap(); // TODO

        let mut context = ParserContext {
            schema: self.schema,
            anchor_info: taken_anchor_info,
            _phantom: &PhantomData,
        };

        // TODO: Add position if position is missing
        let result = parse_any(&mut context, any_type, value, writer);

        let re_taken_anchor_info = context.take_anchor_info();
        self.anchor_info = re_taken_anchor_info;

        result
    }

    fn anchor_info(&mut self) -> &mut Option<AnchorInfo> {
        &mut self.anchor_info
    }

    fn set_anchor_info(&mut self, anchor_info: Option<AnchorInfo>) {
        self.anchor_info = anchor_info;
    }

    fn present_anchor_info(&mut self) -> &mut AnchorInfo {
        if self.anchor_info().is_none() {
            self.set_anchor_info(Some(AnchorInfo::default()));
        }

        if let Some(info) = &mut self.anchor_info {
            return info;
        } else {
            panic!("must never get here")
        }
    }

    fn take_anchor_info(&mut self) -> Option<AnchorInfo> {
        self.anchor_info.take()
    }
}
