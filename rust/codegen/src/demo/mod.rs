pub mod html_writer;
#[cfg(test)]
pub mod test;

use crate::demo::html_writer::HtmlWriter;
use crate::path::Path;
use crate::path::Segment;
use crate::schema::SchemaBuilderReader;
use crate::settings::Settings;
use crate::vec_read::VecRead;
use crate::CodeReceiver;
use crate::Plugin;
use liquesco_common::error::LqError;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::schema_builder::BuildsOwnSchema;

pub struct HtmlCodeGen;

impl Plugin for HtmlCodeGen {
    fn name(&self) -> &str {
        "schema-html-gen"
    }

    fn description(&self) -> &str {
        "Generates HTML documentation for the liquesco schema language."
    }

    fn process<CR>(&self, receiver: &mut CR, _: &Settings) -> Result<(), LqError>
    where
        CR: CodeReceiver,
    {
        let mut builder = SchemaBuilderReader::default();
        let type_ref = AnyType::build_schema(&mut builder);

        let mut html_writer = HtmlWriter::new(&builder);
        html_writer.write(type_ref);

        let vec = html_writer.finish_to_vec()?;

        receiver.add(Path::new(Segment::new("schema.html")), VecRead::from(vec));

        Ok(())
    }
}
