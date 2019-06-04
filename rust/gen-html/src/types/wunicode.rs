use crate::body_writer::Context;
use crate::body_writer::TypedElementWriter;
use crate::html::list_item;
use crate::html::span;
use liquesco_common::error::LqError;
use liquesco_schema::unicode;
use liquesco_schema::unicode::TUnicode;
use minidom::Element;
use std::marker::PhantomData;

pub struct WUnicode<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WUnicode<'a> {
    type T = TUnicode<'a>;

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

        let length_str = match typ.length_type() {
            unicode::LengthType::Byte => "Number of bytes (actual text length depends on encoding)",
            unicode::LengthType::Utf8Byte => {
                "Number of UTF-8 bytes (needs to compute the length when encoding is not UTF-8)"
            }
            unicode::LengthType::ScalarValue => {
                "Unicode scalar values (this is not grapheme clusters)"
            }
        };
        ul.append_child(list_item("Length type", span(length_str)));

        Ok(ul)
    }
}
