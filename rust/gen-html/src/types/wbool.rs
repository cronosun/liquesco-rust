use crate::body_writer::BodyWriter;
use crate::body_writer::Context;
use liquesco_schema::boolean::TBool;
use minidom::Element;
use liquesco_common::error::LqError;

pub struct WBool;

impl<'a> BodyWriter<'a> for WBool {
    type T = TBool<'a>;

    fn write(_: &mut Context<Self::T>) -> Result<Element, LqError> {
        Ok(Element::bare("span"))
    }
}
