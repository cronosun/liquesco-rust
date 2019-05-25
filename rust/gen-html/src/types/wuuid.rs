use liquesco_schema::uuid::TUuid;
use minidom::Element;
use crate::body_writer::Context;
use crate::body_writer::BodyWriter;

pub struct WUuid;

impl BodyWriter for WUuid {
    type T = TUuid;

    fn write(_ : &mut Context<Self::T>) -> Element {
       Element::bare("span")
    }
}