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
use liquesco_schema::types::uint::TUInt;
use crate::types::common::Common;
use liquesco_schema::types::sint::TSInt;
use liquesco_schema::types::tint::TInt;

pub struct WUInt<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WUInt<'a> {
    type T = TUInt<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
        where TContext: ContextProvider<'b> {
        let range = typ.range();
        Ok(vec![
            Row::association_with_text("Min value (inclusive)", Common::fmt_u128(range.start())),
            Row::association_with_text("Max value (inclusive)", Common::fmt_u128(range.end())),
            Row::association_with_text("Memory width", format!("{} bits", typ.memory().number_of_bits()))
        ])
    }
}

pub struct WSInt<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WSInt<'a> {
    type T = TSInt<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
        where TContext: ContextProvider<'b> {
        let range = typ.range();
        Ok(vec![
            Row::association_with_text("Min value (inclusive)", Common::fmt_i128(range.start())),
            Row::association_with_text("Max value (inclusive)", Common::fmt_i128(range.end())),
            Row::association_with_text("Memory width", format!("{} bits", typ.memory().number_of_bits()))
        ])
    }
}
