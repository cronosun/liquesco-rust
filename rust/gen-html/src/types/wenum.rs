use crate::reference::Reference;
use liquesco_schema::enumeration::TEnum;
use std::marker::PhantomData;
use minidom::Element;
use crate::body_writer::Context;
use liquesco_schema::identifier::Format;
use crate::body_writer::BodyWriter;

pub struct WEnum<'a> {
    _phantom : &'a PhantomData<()>
}

impl<'a> BodyWriter for WEnum<'a> {
    type T = TEnum<'a>;
    fn write(ctx : &mut Context<Self::T>) -> Element {
        let mut ol = Element::builder("ol").attr("start", "0").build();
        for variant in ctx.r#type.variants() {
            let mut li = Element::builder("li").build();

            // var
            let mut var = Element::bare("var");
            var.append_text_node(variant.name().to_string(Format::SnakeCase));
            li.append_child(var);

            // maybe values
            let values = variant.values();
            let number_of_values = values.len();
            if number_of_values > 0 {
                for value in values {
                    li.append_child(Element::bare("br"));
                    let type_info = ctx.schema.type_info(*value);
                    let link = Reference {
                        type_info : &type_info,
                        names : &mut ctx.names
                    }.link();
                    li.append_child(link);
                }
            }

            ol.append_child(li);
        }

        ol
    }
}