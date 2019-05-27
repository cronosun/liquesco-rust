use crate::body_writer::BodyWriter;
use crate::body_writer::Context;
use liquesco_schema::boolean::TBool;
use minidom::Element;

pub struct WBool;

impl<'a> BodyWriter<'a> for WBool {
    type T = TBool<'a>;

    fn write(_: &mut Context<Self::T>) -> Element {
        Element::bare("span")
    }
}
