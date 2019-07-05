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
use liquesco_schema::types::decimal::TDecimal;
use crate::types::common::Common;

pub struct WDecimal<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WDecimal<'a> {
    type T = TDecimal<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
        where TContext : ContextProvider<'b> {
        let range = typ.range();
        Ok(vec![
            Row::association_with_text(
                format!("Minimum ({})", included(range.start_included())),
                    Common::fmt_decimal(range.start())),
            Row::association_with_text(
                format!("Maximum ({})", included(range.end_included())),
                Common::fmt_decimal(range.end())),
        ])
    }
}

fn included(included: bool) -> &'static str {
    if included {
        "inclusive"
    } else {
        "exclusive"
    }
}
