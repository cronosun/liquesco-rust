use crate::body_writer::Context;
use crate::body_writer::TypedElementWriter;
use liquesco_common::error::LqError;
use liquesco_schema::types::boolean::TBool;
use minidom::Element;
use std::marker::PhantomData;

pub struct WBool<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WBool<'a> {
    type T = TBool<'a>;

    fn write(_: &Context, _: &Self::T) -> Result<Element, LqError> {
        Ok(Element::bare("span"))
    }
}
