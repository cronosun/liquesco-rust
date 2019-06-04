use crate::body_writer::Context;
use crate::body_writer::ContextFunctions;
use crate::body_writer::TypedElementWriter;
use crate::html::list_item;
use crate::html::span;
use liquesco_common::error::LqError;
use liquesco_schema::range::{Inclusion, TRange};
use minidom::Element;
use std::marker::PhantomData;

pub struct WRange<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WRange<'a> {
    type T = TRange<'a>;

    fn write(ctx: &Context, typ: &Self::T) -> Result<Element, LqError> {
        let mut ul = Element::bare("ul");

        let link = ctx.link_to(typ.element())?;

        let range_element = list_item("Range element", link);
        ul.append_child(range_element);

        let inclusion: (&str, &str) = match typ.inclusion() {
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
            span(if typ.allow_empty() { "Yes" } else { "No" }),
        ));

        Ok(ul)
    }
}
