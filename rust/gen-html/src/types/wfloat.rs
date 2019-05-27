use liquesco_schema::float::TFloat;
use liquesco_schema::float::TFloat64;
use liquesco_schema::float::TFloat32;
use crate::html::span;
use crate::html::list_item;
use minidom::Element;
use crate::body_writer::Context;
use crate::body_writer::BodyWriter;
use std::fmt::Debug;
use std::fmt::Display;
use liquesco_common::range::Range;

pub struct WFloat32;

impl BodyWriter for WFloat32 {
    type T = TFloat32;

    fn write(ctx : &mut Context<Self::T>) -> Element {
               let mut ul = Element::bare("ul");
        let range = ctx.r#type.range();
        float_range(&mut ul, range, std::f32::MIN.into(), std::f32::MAX.into());
        float_properties(&mut ul, ctx.r#type);
        ul
    }
}

pub struct WFloat64;

impl BodyWriter for WFloat64 {
    type T = TFloat64;

    fn write(ctx : &mut Context<Self::T>) -> Element {
               let mut ul = Element::bare("ul");
        let range = ctx.r#type.range();
        float_range(&mut ul, range, std::f64::MIN.into(), std::f64::MAX.into());
        float_properties(&mut ul, ctx.r#type);
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
            value = number_display(*range.start(), min, max)
        )),
    );
    element.append_child(min_len);
    let max_len = list_item(
        "Maximum value",
        span(format!(
            "{value} ({incl})",
            incl = included(range.end_included()),
            value = number_display(*range.end(), min, max)
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