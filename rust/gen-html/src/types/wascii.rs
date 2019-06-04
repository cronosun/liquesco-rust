use crate::body_writer::Context;
use crate::body_writer::TypedElementWriter;
use crate::html::list_item;
use crate::html::span;
use liquesco_common::error::LqError;
use liquesco_schema::ascii::TAscii;
use minidom::Element;
use std::marker::PhantomData;

pub struct WAscii<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WAscii<'a> {
    type T = TAscii<'a>;

    fn write(_: &Context, typ: &Self::T) -> Result<Element, LqError> {
        let mut ul = Element::bare("ul");

        // information about Length
        let length = typ.length();
        let min_len = list_item(
            "Length minimum (inclusive; number of chars)",
            span(format!("{start}", start = length.start())),
        );
        ul.append_child(min_len);
        let max_len = list_item(
            "Length maximum (inclusive; number of chars)",
            span(format!("{end}", end = length.end())),
        );
        ul.append_child(max_len);

        // allowed codes
        let codes = typ.codes();
        let number_of_ranges = codes.len() / 2;
        for index in 0..number_of_ranges {
            let start = codes[index * 2];
            let end = codes[index * 2 + 1];
            let range_info = list_item(
                format!("Allowed code range #{index}", index = index + 1),
                span(format!(
                    "{start} (inclusive) - {end} (exclusive); [{start_ascii}-{end_ascii}] \
                     (inclusive).",
                    start = start,
                    end = end,
                    start_ascii = char::from(start),
                    end_ascii = char::from(end - 1)
                )),
            );
            ul.append_child(range_info);
        }

        Ok(ul)
    }
}
