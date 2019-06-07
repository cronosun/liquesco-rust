use crate::body_writer::Context;
use crate::body_writer::TypedElementWriter;
use crate::html::list_item;
use crate::html::span;
use liquesco_common::error::LqError;
use liquesco_schema::key_ref::TKeyRef;
use minidom::Element;
use std::marker::PhantomData;

pub struct WKeyRef<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WKeyRef<'a> {
    type T = TKeyRef<'a>;

    fn write(_: &Context, typ: &Self::T) -> Result<Element, LqError> {
        let mut ul = Element::bare("ul");
        ul.append_child(list_item("Level", span(format!("{}", typ.level()))));
        // TODO: Also provide information about type directly: Note: Would need to make "context" mut (so the map can add information about that)
        Ok(ul)
    }
}
