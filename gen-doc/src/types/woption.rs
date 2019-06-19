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

pub struct WOption<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WOption<'a> {
    type T = TOption<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
        where TContext : ContextProvider<'b> {
        Ok(vec![
            ctx.named_link_to_type("Present", typ.r#type())?
        ])
    }
}
