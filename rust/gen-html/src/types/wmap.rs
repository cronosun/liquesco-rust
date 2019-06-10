use crate::body_writer::Context;
use crate::body_writer::ContextFunctions;
use crate::body_writer::TypedElementWriter;
use crate::html::list_item;
use crate::html::span;
use liquesco_common::error::LqError;
use liquesco_schema::types::map::Sorting;
use liquesco_schema::types::map::TMap;
use minidom::Element;
use std::marker::PhantomData;

pub struct WMap<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WMap<'a> {
    type T = TMap<'a>;

    fn write(ctx: &Context, typ: &Self::T) -> Result<Element, LqError> {
        let mut ul = Element::bare("ul");

        ul.append_child(list_item("Key type", ctx.link_to(typ.key())?));
        ul.append_child(list_item("Value type", ctx.link_to(typ.value())?));

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

        ul.append_child(list_item(
            "Key sorting",
            span(match typ.sorting() {
                Sorting::Ascending => "Ascending",
                Sorting::Descending => "Descending",
            }),
        ));

        ul.append_child(list_item(
            "Anchors (can be referenced)",
            span(if typ.anchors() { "Yes" } else { "No" }),
        ));

        Ok(ul)
    }
}
