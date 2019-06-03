use crate::body_writer::BodyWriter;
use crate::body_writer::Context;
use liquesco_schema::option::TOption;
use minidom::Element;
use liquesco_common::error::LqError;

pub struct WOption;

impl<'a> BodyWriter<'a> for WOption {
    type T = TOption<'a>;

    fn write(ctx: &mut Context<Self::T>) -> Result<Element, LqError> {
        let mut item = Element::bare("p");
        item.append_text_node("Present type ");

        let link = ctx.link_to(Some(ctx.r#type().r#type()))?;
        item.append_child(link);
        Ok(item)
    }
}
