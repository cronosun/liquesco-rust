use crate::body_writer::BodyWriter;
use crate::body_writer::Context;
use liquesco_schema::enumeration::TEnum;
use liquesco_schema::identifier::Format;
use minidom::Element;
use liquesco_common::error::LqError;

pub struct WEnum;

impl<'a> BodyWriter<'a> for WEnum {
    type T = TEnum<'a>;
    fn write(ctx: &mut Context<Self::T>) -> Result<Element, LqError> {
        let mut ol = Element::builder("ol").attr("start", "0").build();
        for variant in ctx.r#type().variants() {
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
                    let link = ctx.link_to(Some(value))?;
                    li.append_child(link);
                }
            }

            ol.append_child(li);
        }

        ol
    }
}
