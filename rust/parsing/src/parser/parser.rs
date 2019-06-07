use crate::parser::any_parser::parse_any;
use crate::parser::converter::Converter;
use crate::parser::core::AnchorInfo;
use crate::parser::core::Context;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::core::{Schema, TypeRef};
use liquesco_serialization::core::ToVecLqWriter;
use liquesco_serialization::vec_writer::VecWriter;
use std::convert::TryFrom;
use std::marker::PhantomData;

pub(crate) struct ParserContext<'se, 's, TSchema>
where
    TSchema: Schema,
{
    pub(crate) schema: &'se TSchema,
    pub(crate) parent: Option<&'se ParserContext<'se, 's, TSchema>>,
    pub(crate) anchor_info: Vec<AnchorInfo>, // TODO: Try to use smallVec (since in 99% of the cases this is empty or has 1 element)
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
        &self,
        writer: &mut Self::TWriter,
        r#type: &TypeRef,
        value: &TextValue,
    ) -> Result<(), LqError> {
        //let taken_anchor_info = self.anchor_info.take();

        let maybe_any_type = self.schema().maybe_type(r#type);
        let any_type = maybe_any_type.unwrap(); // TODO

        let mut context = ParserContext {
            schema: self.schema,
            parent: Some(self),
            anchor_info: vec![],
            _phantom: &PhantomData,
        };

        // TODO: Add position if position is missing
        let result = parse_any(&mut context, any_type, value, writer);

        //self.anchor_info = context.anchor_info;

        result
    }

    fn parse_to_vec(&self, r#type: &TypeRef, value: &TextValue) -> Result<Vec<u8>, LqError> {
        let mut vec_writer = VecWriter::default();
        self.parse(&mut vec_writer, r#type, value)?;
        Ok(vec_writer.into_vec())
    }

    fn push_anchors(&mut self, anchors: AnchorInfo) {
        self.anchor_info.push(anchors);
    }

    fn pop_anchors(&mut self) -> Result<(), LqError> {
        if self.anchor_info.len() > 0 {
            self.anchor_info.remove(0);
            Ok(())
        } else {
            LqError::err_new("There's a problem with anchor info. You're trying to remove \
            anchor info from stack but the stack is empty. There might be an anchor info in the \
            parent: But one single type parser should only remove the anchors it has put itself.")
        }
    }

    fn anchors(&self, level: u32) -> Option<&AnchorInfo> {
        let self_len = self.anchor_info.len();
        let u32_self_len = u32::try_from(self_len).ok();
        if let Some(self_len) = u32_self_len {
            if level >= self_len {
                self.parent
                    .and_then(|parent| parent.anchors(level - self_len))
            } else {
                let index = usize::try_from(self_len - level - 1).ok();
                index.map(|index| &self.anchor_info[index])
            }
        } else {
            None
        }
    }
}
