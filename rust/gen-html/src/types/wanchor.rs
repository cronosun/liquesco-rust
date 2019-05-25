use crate::body_writer::BodyWriter;
use crate::body_writer::Context;
use liquesco_schema::anchors::TAnchors;
use liquesco_schema::reference::TReference;
use minidom::Element;

pub struct WAnchors;

impl BodyWriter for WAnchors {
    type T = TAnchors;

    fn write(_: &mut Context<Self::T>) -> Element {
        let mut element = Element::bare("div");
        element.append_text_node("Not yet implemented");
        element
    }
}

pub struct WReference;

impl BodyWriter for WReference {
    type T = TReference;

    fn write(_: &mut Context<Self::T>) -> Element {
        Element::bare("span")
    }
}