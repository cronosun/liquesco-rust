use crate::body_writer::Context;
use crate::body_writer::ContextFunctions;
use crate::body_writer::TypedElementWriter;
use crate::html::list_item;
use crate::html::span;
use liquesco_common::error::LqError;
use liquesco_schema::types::seq;
use liquesco_schema::types::seq::TSeq;
use minidom::Element;
use std::marker::PhantomData;

pub struct WSeq<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WSeq<'a> {
    type T = TSeq<'a>;

    fn write(ctx: &Context, typ: &Self::T) -> Result<Element, LqError> {
        let mut ul = Element::bare("ul");

        let element = list_item("Element type", ctx.link_to(typ.element())?);
        ul.append_child(element);

        // information about length
        let length = typ.length();
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
            let fix_len = list_item("Fixed length", span(format!("{len}", len = length.start())));
            ul.append_child(fix_len);
        }
        if let Some(multiple_of) = typ.multiple_of() {
            let max_len = list_item(
                "Length multiple of",
                span(format!("{mult_of}", mult_of = multiple_of)),
            );
            ul.append_child(max_len);
        }

        // ordering
        let ordering = typ.ordering();
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
            seq::Ordering::Sorted(value) => {
                let ordering = list_item(
                    "Sorting direction",
                    match value.direction {
                        seq::Direction::Ascending => span("Ascending (required sorting)"),
                        seq::Direction::Descending => span("Descending (required sorting)"),
                    },
                );
                ul.append_child(ordering);
                if value.unique {
                    let unique_li = list_item("Unique", span("Duplicate elements are not allowed"));
                    ul.append_child(unique_li);
                }
            }
        }

        Ok(ul)
    }
}