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
use liquesco_schema::types::ascii::TAscii;
use liquesco_schema::types::uuid::TUuid;

pub struct WUuid<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WUuid<'a> {
    type T = TUuid<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
        where TContext: ContextProvider<'b> {
        Ok(Vec::new())
    }
}
