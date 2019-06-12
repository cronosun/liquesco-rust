use crate::body_writer::Context;
use crate::body_writer::TypedElementWriter;
use crate::html::list_item;
use crate::html::span;
use liquesco_common::error::LqError;
use liquesco_schema::types::decimal::TDecimal;
use minidom::Element;
use std::marker::PhantomData;

pub struct WDecimal<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WDecimal<'a> {
    type T = TDecimal<'a>;

    fn write(_: &Context, typ: &Self::T) -> Result<Element, LqError> {
        let mut ul = Element::bare("ul");

        ul.append_child(list_item(
            "Minimum value",
            span(format!(
                "{value} ({included})",
                value = typ.range().start(),
                included = included(typ.range().start_included())
            )),
        ));
        ul.append_child(list_item(
            "Maximum value",
            span(format!(
                "{value} ({included})",
                value = typ.range().end(),
                included = included(typ.range().end_included())
            )),
        ));

        Ok(ul)
    }
}

fn included(included: bool) -> &'static str {
    if included {
        "inclusive"
    } else {
        "exclusive"
    }
}
