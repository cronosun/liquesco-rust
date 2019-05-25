use std::marker::PhantomData;
use minidom::Element;
use crate::body_writer::Context;
use liquesco_schema::structure::TStruct;
use liquesco_schema::identifier::Format;
use crate::body_writer::BodyWriter;

pub struct WStruct<'a> {
    _phantom : &'a PhantomData<()>
}

impl<'a> BodyWriter for WStruct<'a> {
    type T = TStruct<'a>;
    fn write(ctx : &mut Context<Self::T>) -> Element {
        
         let mut ol = Element::builder("ol").attr("start", "0").build();
        for field in ctx.r#type.fields() {
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

            ctx.set_uses(field.r#type());
        }

        ol
    }
}