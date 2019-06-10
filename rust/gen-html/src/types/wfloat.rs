use crate::body_writer::Context;
use crate::body_writer::TypedElementWriter;
use crate::html::list_item;
use crate::html::span;
use liquesco_common::error::LqError;
use liquesco_common::range::Range;
use liquesco_schema::types::float::TFloat;
use liquesco_schema::types::float::TFloat32;
use liquesco_schema::types::float::TFloat64;
use minidom::Element;
use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;

pub struct WFloat32<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WFloat32<'a> {
    type T = TFloat32<'a>;

    fn write(_: &Context, typ: &Self::T) -> Result<Element, LqError> {
        let mut ul = Element::bare("ul");
        let range = typ.range();
        float_range(&mut ul, range, std::f32::MIN.into(), std::f32::MAX.into());
        float_properties(&mut ul, typ);
        Ok(ul)
    }
}

pub struct WFloat64<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WFloat64<'a> {
    type T = TFloat64<'a>;

    fn write(_: &Context, typ: &Self::T) -> Result<Element, LqError> {
        let mut ul = Element::bare("ul");
        let range = typ.range();
        float_range(&mut ul, range, std::f64::MIN.into(), std::f64::MAX.into());
        float_properties(&mut ul, typ);
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
        span(yes_no(float.allow_nan())),
    ));
    element.append_child(list_item(
        "Allow positive infinity",
        span(yes_no(float.allow_positive_infinity())),
    ));
    element.append_child(list_item(
        "Allow negative infinity",
        span(yes_no(float.allow_negative_infinity())),
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
