use liquesco_schema::boolean::TBool;
use minidom::Element;
use crate::body_writer::Context;
use crate::body_writer::BodyWriter;

pub struct WBool;

impl BodyWriter for WBool {
    type T = TBool;

    fn write(_ : &mut Context<Self::T>) -> Element {
       Element::bare("span")
    }
}