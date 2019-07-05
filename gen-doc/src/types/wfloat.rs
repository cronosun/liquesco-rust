use crate::context::{Context, ContextProvider};
use crate::context::ContextFunctions;
use liquesco_common::error::LqError;
use liquesco_processing::type_info::TypeInfo;
use liquesco_schema::types::option::TOption;
use minidom::Element;
use std::marker::PhantomData;
use crate::type_writer::TypeBodyWriter;
use crate::model::row::{Row, Link};
use crate::model::row;
use crate::model::card::CardId;
use liquesco_schema::types::boolean::TBool;
use liquesco_schema::types::float::{TFloat32, TFloat, TFloat64};
use std::fmt::Debug;
use crate::types::common::Common;
use liquesco_common::range::Range;

pub struct WFloat32<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WFloat32<'a> {
    type T = TFloat32<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
        where TContext : ContextProvider<'b> {

        let mut rows = Vec::new();
        float_range(&mut rows, typ.range(),
                    Common::fmt_f32(typ.range().start()),
                    Common::fmt_f32(typ.range().end()));
        float_properties(&mut rows, typ);
        Ok(rows)
    }
}

pub struct WFloat64<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WFloat64<'a> {
    type T = TFloat64<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
        where TContext : ContextProvider<'b> {

        let mut rows = Vec::new();
        float_range(&mut rows, typ.range(),
                    Common::fmt_f64(typ.range().start()),
                    Common::fmt_f64(typ.range().end()));
        float_properties(&mut rows, typ);
        Ok(rows)
    }
}

fn float_range<F>(rows : &mut Vec<Row>, range : &Range<F>, start : String, end : String) {
    rows.push(
        Row::association_with_text("Min value",
                                   start));
    rows.push(
        Row::association_with_text("Min included",
                                   Common::fmt_bool_yes_no(range.start_included())));
    rows.push(
        Row::association_with_text("Max value",
                                   end));
    rows.push(
        Row::association_with_text("Max included",
                                   Common::fmt_bool_yes_no(range.end_included())));
}

fn float_properties<F>(rows : &mut Vec<Row>, float: &TFloat<F>)
    where
        F: Eq + PartialOrd + Debug,
{
    rows.push(
        Row::association_with_text("Allow positive zero",
        Common::fmt_bool_yes_no(float.allow_positive_zero())));
    rows.push(
        Row::association_with_text("Allow negative zero",
                                   Common::fmt_bool_yes_no(float.allow_negative_zero())));
    rows.push(
        Row::association_with_text("Allow NaN (not a number)",
                                   Common::fmt_bool_yes_no(float.allow_nan())));
    rows.push(
        Row::association_with_text("Allow positive infinity",
                                   Common::fmt_bool_yes_no(float.allow_positive_infinity())));
    rows.push(
        Row::association_with_text("Allow negative infinity",
                                   Common::fmt_bool_yes_no(float.allow_negative_infinity())));
    rows.push(
        Row::association_with_text("Allow subnormal",
                                   Common::fmt_bool_yes_no(float.allow_subnormal())));
}