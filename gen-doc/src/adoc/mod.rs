use crate::adoc::model_writer::ModelWriter;
use crate::model::Model;
use liquesco_processing::text::Text;
use liquesco_common::error::LqError;

mod card_writer;
mod model_writer;
mod section_writer;

pub struct AsciiDoc {}

impl AsciiDoc {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write_to(&self, model: &Model, text: &mut Text) -> Result<(), LqError> {
        let mut model_writer = ModelWriter::new(model, text);
        model_writer.write()
    }
}
