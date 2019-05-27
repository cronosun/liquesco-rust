use crate::body_writer::BodyWriter;
use crate::body_writer::Context;
use crate::reference::Reference;
use liquesco_schema::option::TOption;
use minidom::Element;

pub struct WOption;

impl BodyWriter for WOption {
    type T = TOption;

    fn write(ctx: &mut Context<Self::T>) -> Element {
        let mut item = Element::bare("p");
        item.append_text_node("Present type ");

        let type_info = ctx.schema.type_info(ctx.r#type.r#type());
        let link = Reference {
            type_info: &type_info,
            names: &mut ctx.names,
        }
        .link();

        item.append_child(link);
        item
    }
}
