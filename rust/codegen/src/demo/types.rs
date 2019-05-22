use liquesco_schema::reference::TReference;
use crate::demo::html_writer::HtmlWriter;
use liquesco_common::range::Range;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::ascii::TAscii;
use liquesco_schema::boolean::TBool;
use liquesco_schema::core::TypeRef;
use liquesco_schema::enumeration::TEnum;
use liquesco_schema::float::TFloat;
use liquesco_schema::float::TFloat32;
use liquesco_schema::float::TFloat64;
use liquesco_schema::identifier::Format;
use liquesco_schema::option::TOption;
use liquesco_schema::seq;
use liquesco_schema::seq::TSeq;
use liquesco_schema::sint::TSInt;
use liquesco_schema::structure::TStruct;
use liquesco_schema::uint::TUInt;
use liquesco_schema::unicode;
use liquesco_schema::unicode::TUnicode;
use minidom::Element;
use std::fmt::Debug;
use std::fmt::Display;

impl<'a> HtmlWriter<'a> {
    pub(crate) fn type_body(&mut self, any_type: &AnyType, type_ref: TypeRef) -> Element {
        match any_type {
            AnyType::Enum(value) => self.type_body_enum(value, type_ref),
            AnyType::Struct(value) => self.type_body_struct(value, type_ref),
            AnyType::Option(value) => self.type_body_option(value, type_ref),
            AnyType::Seq(value) => self.type_body_seq(value, type_ref),
            AnyType::Ascii(value) => self.type_body_ascii(value, type_ref),
            AnyType::UInt(value) => self.type_body_uint(value, type_ref),
            AnyType::SInt(value) => self.type_body_sint(value, type_ref),
            AnyType::Float32(value) => self.type_body_f32(value, type_ref),
            AnyType::Float64(value) => self.type_body_f64(value, type_ref),
            AnyType::Bool(value) => self.type_body_bool(value, type_ref),
            AnyType::Unicode(value) => self.type_body_unicode(value, type_ref),
            AnyType::Reference(value) => self.type_body_ref(value, type_ref),
            _ => {
                let mut element = Element::bare("p");
                element.append_text_node("TODO: The text");
                element
            }
        }
    }
}

/// For enums
impl<'a> HtmlWriter<'a> {
    pub(crate) fn type_body_enum(&mut self, value: &TEnum, _: TypeRef) -> Element {
        let mut ol = Element::builder("ol").attr("start", "0").build();
        for variant in value.variants() {
            let mut li = Element::builder("li").build();

            // var
            let mut var = Element::bare("var");
            var.append_text_node(variant.name().to_string(Format::SnakeCase));
            li.append_child(var);

            // maybe values
            let values = variant.values();
            let number_of_values = values.len();
            if number_of_values > 0 {
                for value in values {
                    li.append_child(Element::bare("br"));
                    let value_any_type = self.schema.require(*value);
                    li.append_child(self.ref_link(value_any_type, *value));
                    self.write(*value);
                }
            }

            ol.append_child(li);
        }

        ol
    }
}

/// For struct
impl<'a> HtmlWriter<'a> {
    pub(crate) fn type_body_struct(&mut self, value: &TStruct, _: TypeRef) -> Element {
        let mut ol = Element::builder("ol").attr("start", "0").build();
        for field in value.fields() {
            let mut li = Element::builder("li").build();

            // var
            let mut var = Element::bare("var");
            var.append_text_node(field.name().to_string(Format::SnakeCase));
            li.append_child(var);

            let mut space = Element::bare("span");
            space.append_text_node(": ");
            li.append_child(space);

            // value
            let value_any_type = self.schema.require(field.r#type());
            li.append_child(self.ref_link(value_any_type, field.r#type()));

            ol.append_child(li);

            self.write(field.r#type());
        }

        ol
    }
}

/// For option
impl<'a> HtmlWriter<'a> {
    pub(crate) fn type_body_option(&mut self, value: &TOption, _: TypeRef) -> Element {
        let mut item = Element::bare("p");
        item.append_text_node("Present type ");

        let value_any_type = self.schema.require(value.r#type());
        item.append_child(self.ref_link(value_any_type, value.r#type()));

        self.write(value.r#type());

        item
    }
}

fn list_item<D: Into<String>>(definition: D, value: Element) -> Element {
    let mut def = Element::bare("strong");
    def.append_text_node(definition);

    let mut space = Element::bare("span");
    space.append_text_node(": ");

    Element::builder("li")
        .append(def)
        .append(space)
        .append(value)
        .build()
}

fn span<D: Into<String>>(text: D) -> Element {
    let mut div = Element::bare("span");
    div.append_text_node(text);
    div
}

/// For sequence
impl<'a> HtmlWriter<'a> {
    pub(crate) fn type_body_seq(&mut self, value: &TSeq, _: TypeRef) -> Element {
        let mut ul = Element::bare("ul");

        let element_any_type = self.schema.require(value.element());
        let element = list_item(
            "Element type",
            self.ref_link(element_any_type, value.element()),
        );
        ul.append_child(element);
        self.write(value.element());

        // information about length
        let length = value.length();
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
        if let Some(multiple_of) = value.multiple_of() {
            let max_len = list_item(
                "Length multiple of",
                span(format!("{mult_of}", mult_of = multiple_of)),
            );
            ul.append_child(max_len);
        }

        // ordering
        let ordering = value.ordering();
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

/// For acii
impl<'a> HtmlWriter<'a> {
    pub(crate) fn type_body_ascii(&mut self, value: &TAscii, _: TypeRef) -> Element {
        let mut ul = Element::bare("ul");

        // information about Length
        let length = value.length();
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
        let codes = value.codes();
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

        ul
    }
}

/// For unsigned int
impl<'a> HtmlWriter<'a> {
    pub(crate) fn type_body_uint(&mut self, value: &TUInt, _: TypeRef) -> Element {
        let mut ul = Element::bare("ul");

        let min_len = list_item(
            "Minimum value (inclusive)",
            span(format!("{value}", value = value.range().start())),
        );
        ul.append_child(min_len);
        let max_len = list_item(
            "Maximum value (inclusive)",
            span(format!("{value}", value = value.range().end())),
        );
        ul.append_child(max_len);

        ul
    }
}

/// For signed int
impl<'a> HtmlWriter<'a> {
    pub(crate) fn type_body_sint(&mut self, value: &TSInt, _: TypeRef) -> Element {
        let mut ul = Element::bare("ul");

        let min_len = list_item(
            "Minimum value (inclusive)",
            span(format!("{value}", value = value.range().start())),
        );
        ul.append_child(min_len);
        let max_len = list_item(
            "Maximum value (inclusive)",
            span(format!("{value}", value = value.range().end())),
        );
        ul.append_child(max_len);

        ul
    }
}

fn included(included: bool) -> &'static str {
    if included {
        "inclusive"
    } else {
        "exclusive"
    }
}

fn yes_no(yes: bool) -> &'static str {
    if yes {
        "Yes"
    } else {
        "No"
    }
}

fn float_range<F>(element: &mut Element, range: &Range<F>, min: F, max: F)
where
    F: Display + Eq + Copy,
{
    let min_len = list_item(
        "Minimum value",
        span(format!(
            "{value} ({incl})",
            incl = included(range.start_included()),
            value = number_display(*range.bounds().start(), min, max)
        )),
    );
    element.append_child(min_len);
    let max_len = list_item(
        "Maximum value",
        span(format!(
            "{value} ({incl})",
            incl = included(range.end_included()),
            value = number_display(*range.bounds().end(), min, max)
        )),
    );
    element.append_child(max_len);
}

fn float_properties<F>(element: &mut Element, float: &TFloat<F>)
where
    F: Eq + PartialOrd + Debug,
{
    element.append_child(list_item(
        "Allow NaN (not a number)",
        span(yes_no(float.allow_nan)),
    ));
    element.append_child(list_item(
        "Allow positive infinity",
        span(yes_no(float.allow_positive_infinity)),
    ));
    element.append_child(list_item(
        "Allow negative infinity",
        span(yes_no(float.allow_negative_infinity)),
    ));
}

fn number_display<T>(value: T, min: T, max: T) -> String
where
    T: PartialEq + Display,
{
    if value == min {
        "Minimum".to_string()
    } else if value == max {
        "Maximum".to_string()
    } else {
        format!("{value}", value = value)
    }
}

/// float32
impl<'a> HtmlWriter<'a> {
    pub(crate) fn type_body_f32(&mut self, value: &TFloat32, _: TypeRef) -> Element {
        let mut ul = Element::bare("ul");
        let range = value.range();
        float_range(&mut ul, range, std::f32::MIN.into(), std::f32::MAX.into());
        float_properties(&mut ul, value);
        ul
    }
}

/// float64
impl<'a> HtmlWriter<'a> {
    pub(crate) fn type_body_f64(&mut self, value: &TFloat64, _: TypeRef) -> Element {
        let mut ul = Element::bare("ul");
        let range = value.range();
        float_range(&mut ul, range, std::f64::MIN.into(), std::f64::MAX.into());
        float_properties(&mut ul, value);
        ul
    }
}

/// bool
impl<'a> HtmlWriter<'a> {
    pub(crate) fn type_body_bool(&mut self, _: &TBool, _: TypeRef) -> Element {
        Element::bare("span")
    }
}

/// reference
impl<'a> HtmlWriter<'a> {
    pub(crate) fn type_body_ref(&mut self, _: &TReference, _: TypeRef) -> Element {
        Element::bare("span")
    }
}

/// For unicode
impl<'a> HtmlWriter<'a> {
    pub(crate) fn type_body_unicode(&mut self, value: &TUnicode, _: TypeRef) -> Element {
        let mut ul = Element::bare("ul");

        let min_len = list_item(
            "Minimum length (inclusive)",
            span(format!("{value}", value = value.length().start())),
        );
        ul.append_child(min_len);
        let max_len = list_item(
            "Maximum length (inclusive)",
            span(format!("{value}", value = value.length().end())),
        );
        ul.append_child(max_len);

        let length_str = match value.length_type() {
            unicode::LengthType::Byte => "Number of bytes (actual text length depends on encoding)",
            unicode::LengthType::Utf8Byte => {
                "Number of UTF-8 bytes (needs to compute the length when encoding is not UTF-8)"
            }
            unicode::LengthType::ScalarValue => {
                "Unicode scalar values (this is not grapheme clusters)"
            }
        };
        ul.append_child(list_item("Length type", span(length_str)));

        ul
    }
}
