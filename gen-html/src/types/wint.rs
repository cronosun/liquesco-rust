use crate::body_writer::Context;
use crate::body_writer::TypedElementWriter;
use crate::html::list_item;
use crate::html::span;
use liquesco_common::error::LqError;
use liquesco_schema::types::sint::TSInt;
use liquesco_schema::types::tint::TInt;
use liquesco_schema::types::uint::TUInt;
use minidom::Element;
use std::marker::PhantomData;

pub struct WUInt<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WUInt<'a> {
    type T = TUInt<'a>;

    fn write(_: &Context, typ: &Self::T) -> Result<Element, LqError> {
        let mut ul = Element::bare("ul");

        let min_len = list_item(
            "Minimum value (inclusive)",
            span(format!("{value}", value = typ.range().start())),
        );
        ul.append_child(min_len);
        let max_len = list_item(
            "Maximum value (inclusive)",
            span(format!("{value}", value = typ.range().end())),
        );
        ul.append_child(max_len);

        Ok(ul)
    }
}

pub struct WSInt<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WSInt<'a> {
    type T = TSInt<'a>;

    fn write(_: &Context, typ: &Self::T) -> Result<Element, LqError> {
        let mut ul = Element::bare("ul");

        let min_len = list_item(
            "Minimum value (inclusive)",
            span(format!("{value}", value = typ.range().start())),
        );
        ul.append_child(min_len);
        let max_len = list_item(
            "Maximum value (inclusive)",
            span(format!("{value}", value = typ.range().end())),
        );
        ul.append_child(max_len);

        Ok(ul)
    }
}
