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
use liquesco_schema::types::binary::TBinary;
use crate::types::common::Common;

pub struct WBinary<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WBinary<'a> {
    type T = TBinary<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
        where TContext : ContextProvider<'b> {
        Ok(vec![
            Row::association_with_text("Minimum length (inclusive)",
                Common::fmt_u64(*typ.length().start())),
            Row::association_with_text("Maximum length (inclusive)",
                                       Common::fmt_u64(*typ.length().end()))
        ])
    }
}
