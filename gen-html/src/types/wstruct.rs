use crate::body_writer::Context;
use crate::body_writer::ContextFunctions;
use crate::body_writer::TypedElementWriter;
use liquesco_common::error::LqError;
use liquesco_schema::identifier::Format;
use liquesco_schema::types::structure::TStruct;
use minidom::Element;
use std::marker::PhantomData;

pub struct WStruct<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WStruct<'a> {
    type T = TStruct<'a>;
    fn write(ctx: &Context, typ: &Self::T) -> Result<Element, LqError> {
        let mut ol = Element::builder("ol").attr("start", "0").build();
        for field in typ.fields() {
            let mut li = Element::builder("li").build();

            // var
            let mut var = Element::bare("var");
            var.append_text_node(field.name().to_string(Format::SnakeCase));
            li.append_child(var);

            let mut space = Element::bare("span");
            space.append_text_node(": ");
            li.append_child(space);

            // value
            li.append_child(ctx.link_to(field.r#type())?);

            ol.append_child(li);
        }

        Ok(ol)
    }
}
