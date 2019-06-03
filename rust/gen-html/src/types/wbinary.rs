use crate::body_writer::BodyWriter;
use crate::body_writer::Context;
use crate::html::list_item;
use crate::html::span;
use liquesco_schema::binary::TBinary;
use minidom::Element;
use liquesco_common::error::LqError;

pub struct WBinary;

impl<'a> BodyWriter<'a> for WBinary {
    type T = TBinary<'a>;

    fn write(ctx: &mut Context<Self::T>) -> Result<Element, LqError> {
        let mut ul = Element::bare("ul");

        let min_len = list_item(
            "Minimum length (inclusive)",
            span(format!("{value}", value = ctx.r#type().length().start())),
        );
        ul.append_child(min_len);
        let max_len = list_item(
            "Maximum length (inclusive)",
            span(format!("{value}", value = ctx.r#type().length().end())),
        );
        ul.append_child(max_len);

        Ok(ul)
    }
}
