use crate::context::{Context, ContextProvider};
use crate::context::ContextFunctions;
use liquesco_common::error::LqError;
use liquesco_processing::type_info::TypeInfo;
use liquesco_schema::types::option::TOption;
use minidom::Element;
use std::marker::PhantomData;
use crate::type_writer::TypeBodyWriter;
use crate::model::row::{Row, TextWithLink};
use crate::model::row;
use crate::model::card::CardId;
use liquesco_schema::types::root_map::TRootMap;

pub struct WRootMap<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WRootMap<'a> {
    type T = TRootMap<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
        where TContext : ContextProvider<'b> {
        Ok(vec![
            ctx.named_link_to_type("Root type", typ.root())?,
            ctx.named_link_to_type("Key type", typ.key())?,
            ctx.named_link_to_type("Value type", typ.value())?,
            // TODO: Some information missing
        ])
    }
}
