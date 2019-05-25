use crate::html::span;
use crate::html::list_item;
use minidom::Element;
use crate::body_writer::Context;
use liquesco_schema::seq::TSeq;
use liquesco_schema::seq;
use crate::body_writer::BodyWriter;

pub struct WSeq;

impl BodyWriter for WSeq {
    type T = TSeq;

    fn write(ctx : &mut Context<Self::T>) -> Element {
         let mut ul = Element::bare("ul");

        let element = list_item(
            "Element type",
            ctx.link(ctx.r#type.element())
        );
        ul.append_child(element);
        ctx.set_uses(ctx.r#type.element());

        // information about length
        let length = ctx.r#type.length();
        if length.start() != length.end() {
            let min_len = list_item(
                "Length minimum (inclusive)",
                span(format!("{start}", start = length.start())),
            );
            ul.append_child(min_len);
            let max_len = list_item(
                "Length maximum (inclusive)",
                span(format!("{end}", end = length.end())),
            );
            ul.append_child(max_len);
        } else {
            let fix_len = list_item("Fixed length", 
            span(format!("{len}", 
            len = length.start())));
            ul.append_child(fix_len);
        }
        if let Some(multiple_of) = ctx.r#type.multiple_of() {
            let max_len = list_item(
                "Length multiple of",
                span(format!("{mult_of}", mult_of = multiple_of)),
            );
            ul.append_child(max_len);
        }

        // ordering
        let ordering = ctx.r#type.ordering();
        match ordering {
            seq::Ordering::None => {
                let ordering = list_item(
                    "Ordering",
                    span(
                        "No special ordering requirements; \
                         duplicate elements are allowed.",
                    ),
                );
                ul.append_child(ordering);
            }
            seq::Ordering::Sorted { direction, unique } => {
                let ordering = list_item(
                    "Sorting direction",
                    match direction {
                        seq::Direction::Ascending => span("Ascending (required sorting)"),
                        seq::Direction::Descending => span("Descending (required sorting)"),
                    },
                );
                ul.append_child(ordering);
                if *unique {
                    let unique_li = list_item("Unique", span("Duplicate elements are not allowed"));
                    ul.append_child(unique_li);
                }
            }
        }

        ul
    }
}