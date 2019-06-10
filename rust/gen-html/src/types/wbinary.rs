use crate::body_writer::Context;
use crate::body_writer::TypedElementWriter;
use crate::html::list_item;
use crate::html::span;
use liquesco_common::error::LqError;
use liquesco_schema::types::binary::TBinary;
use minidom::Element;
use std::marker::PhantomData;

pub struct WBinary<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WBinary<'a> {
    type T = TBinary<'a>;

    fn write(_: &Context, typ: &Self::T) -> Result<Element, LqError> {
        let mut ul = Element::bare("ul");

        let min_len = list_item(
            "Minimum length (inclusive)",
            span(format!("{value}", value = typ.length().start())),
        );
        ul.append_child(min_len);
        let max_len = list_item(
            "Maximum length (inclusive)",
            span(format!("{value}", value = typ.length().end())),
        );
        ul.append_child(max_len);

        Ok(ul)
    }
}
