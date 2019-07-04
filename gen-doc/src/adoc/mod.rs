use liquesco_processing::text::Text;
use crate::model::Model;
use crate::adoc::model_writer::ModelWriter;

mod card_writer;
mod model_writer;

pub struct AsciiDoc {
}

impl AsciiDoc {

    pub fn new() -> Self {
        Self {}
    }

    pub fn write_to(&self, model : &Model, text : &mut Text) {
        let mut model_writer = ModelWriter::new(model, text);
        model_writer.write()
    }
}

