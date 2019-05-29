use crate::body_writer::BodyWriter;
use crate::body_writer::Context;
use crate::html::list_item;
use crate::html::span;
use crate::reference::Reference;
use liquesco_schema::range::{Inclusion, TRange};
use minidom::Element;

pub struct WRange;

impl<'a> BodyWriter<'a> for WRange {
    type T = TRange<'a>;

    fn write(ctx: &mut Context<Self::T>) -> Element {
        let mut ul = Element::bare("ul");

        let type_info = ctx.schema.type_info(ctx.r#type.element());
        let link = Reference {
            type_info: &type_info,
            names: &mut ctx.names,
        }
        .link();

        let range_element = list_item("Range element", link);
        ul.append_child(range_element);

        let inclusion: (&str, &str) = match ctx.r#type.inclusion() {
            Inclusion::BothInclusive => ("Inclusive", "Inclusive"),
            Inclusion::StartInclusive => ("Inclusive", "Exclusive"),
            Inclusion::BothExclusive => ("Exclusive", "Exclusive"),
            Inclusion::EndInclusive => ("Exclusive", "Inclusive"),
            Inclusion::Supplied => ("Supplied (by data)", "Supplied (by data)"),
        };

        ul.append_child(list_item("Start inclusive", span(inclusion.0)));

        ul.append_child(list_item("End inclusive", span(inclusion.1)));

        ul.append_child(list_item(
            "Allow empty range",
            span(if ctx.r#type.allow_empty() {
                "Yes"
            } else {
                "No"
            }),
        ));

        ul
    }
}
