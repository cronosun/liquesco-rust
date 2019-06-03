use crate::body_writer::BodyWriter;
use crate::body_writer::Context;
use liquesco_schema::identifier::Format;
use liquesco_schema::structure::TStruct;
use minidom::Element;
use liquesco_common::error::LqError;

pub struct WStruct;

impl<'a> BodyWriter<'a> for WStruct {
    type T = TStruct<'a>;
    fn write(ctx: &mut Context<Self::T>) -> Result<Element, LqError> {
        let mut ol = Element::builder("ol").attr("start", "0").build();
        for field in ctx.r#type().fields() {
            let mut li = Element::builder("li").build();

            // var
            let mut var = Element::bare("var");
            var.append_text_node(field.name().to_string(Format::SnakeCase));
            li.append_child(var);

            let mut space = Element::bare("span");
            space.append_text_node(": ");
            li.append_child(space);

            // value
            li.append_child(ctx.link(field.r#type()));

            ol.append_child(li);
        }

        Ok(ol)
    }
}
