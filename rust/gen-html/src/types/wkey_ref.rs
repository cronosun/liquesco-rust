use crate::body_writer::Context;
use crate::body_writer::TypedElementWriter;
use liquesco_common::error::LqError;
use liquesco_schema::key_ref::TKeyRef;
use minidom::Element;
use std::marker::PhantomData;

pub struct WKeyRef<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WKeyRef<'a> {
    type T = TKeyRef<'a>;

    fn write(_: &Context, _: &Self::T) -> Result<Element, LqError> {
        Ok(Element::bare("span"))
    }
}
