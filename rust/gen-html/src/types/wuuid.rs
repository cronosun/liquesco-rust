use crate::body_writer::BodyWriter;
use crate::body_writer::Context;
use liquesco_schema::uuid::TUuid;
use minidom::Element;
use liquesco_common::error::LqError;

pub struct WUuid;

impl<'a> BodyWriter<'a> for WUuid {
    type T = TUuid<'a>;

    fn write(_: &mut Context<Self::T>) -> Result<Element, LqError> {
        Ok(Element::bare("span"))
    }
}
