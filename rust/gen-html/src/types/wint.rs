use crate::body_writer::BodyWriter;
use crate::body_writer::Context;
use crate::html::list_item;
use crate::html::span;
use liquesco_schema::sint::TSInt;
use liquesco_schema::uint::TUInt;
use minidom::Element;

pub struct WUInt;

impl BodyWriter for WUInt {
    type T = TUInt;

    fn write(ctx: &mut Context<Self::T>) -> Element {
        let mut ul = Element::bare("ul");

        let min_len = list_item(
            "Minimum value (inclusive)",
            span(format!("{value}", value = ctx.r#type.range().start())),
        );
        ul.append_child(min_len);
        let max_len = list_item(
            "Maximum value (inclusive)",
            span(format!("{value}", value = ctx.r#type.range().end())),
        );
        ul.append_child(max_len);

        ul
    }
}

pub struct WSInt;

impl BodyWriter for WSInt {
    type T = TSInt;

    fn write(ctx: &mut Context<Self::T>) -> Element {
        let mut ul = Element::bare("ul");

        let min_len = list_item(
            "Minimum value (inclusive)",
            span(format!("{value}", value = ctx.r#type.range().start())),
        );
        ul.append_child(min_len);
        let max_len = list_item(
            "Maximum value (inclusive)",
            span(format!("{value}", value = ctx.r#type.range().end())),
        );
        ul.append_child(max_len);

        ul
    }
}
