use crate::body_writer::Context;
use crate::body_writer::ContextFunctions;
use crate::body_writer::TypedElementWriter;
use liquesco_common::error::LqError;
use liquesco_schema::types::option::TOption;
use minidom::Element;
use std::marker::PhantomData;

pub struct WOption<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WOption<'a> {
    type T = TOption<'a>;

    fn write(ctx: &Context, typ: &Self::T) -> Result<Element, LqError> {
        let mut item = Element::bare("p");
        item.append_text_node("Present type ");

        let link = ctx.link_to(typ.r#type())?;
        item.append_child(link);
        Ok(item)
    }
}
