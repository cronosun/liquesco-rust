pub mod html_writer;
pub mod type_info;
pub mod types;
#[cfg(test)]
pub mod test;
pub mod references;
pub mod usage;

use crate::demo::html_writer::HtmlWriter;
use crate::path::Path;
use crate::path::Segment;
use crate::schema::SchemaBuilderReader;
use crate::code_receiver::Code;
use crate::settings::Settings;
use crate::code_receiver::CodeReceiver;
use crate::plugin::Plugin;
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

    fn process(&self, receiver: &mut CodeReceiver, _: &Settings) -> Result<(), LqError>
    {
        let mut builder = SchemaBuilderReader::default();
        let type_ref = AnyType::build_schema(&mut builder);

        let html_writer = HtmlWriter::new(&builder);
        let string = html_writer.finish_to_string(type_ref)?;

        receiver.add(Path::new(Segment::new("schema.html")), 
            Code::String(string));

        Ok(())
    }
}
