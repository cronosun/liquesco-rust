use crate::body_writer::Context;
use crate::body_writer::ContextFunctions;
use crate::body_writer::TypedElementWriter;
use liquesco_common::error::LqError;
use liquesco_schema::types::enumeration::TEnum;
use liquesco_schema::identifier::Format;
use minidom::Element;
use std::marker::PhantomData;

pub struct WEnum<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WEnum<'a> {
    type T = TEnum<'a>;
    fn write(ctx: &Context, typ: &Self::T) -> Result<Element, LqError> {
        let mut ol = Element::builder("ol").attr("start", "0").build();
        for variant in typ.variants() {
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
                    let link = ctx.link_to(value)?;
                    li.append_child(link);
                }
            }

            ol.append_child(li);
        }

        Ok(ol)
    }
}
