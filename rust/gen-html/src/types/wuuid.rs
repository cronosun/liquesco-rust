use crate::body_writer::Context;
use crate::body_writer::TypedElementWriter;
use liquesco_common::error::LqError;
use liquesco_schema::types::uuid::TUuid;
use minidom::Element;
use std::marker::PhantomData;

pub struct WUuid<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypedElementWriter for WUuid<'a> {
    type T = TUuid<'a>;

    fn write(_: &Context, _: &Self::T) -> Result<Element, LqError> {
        Ok(Element::bare("span"))
    }
}
