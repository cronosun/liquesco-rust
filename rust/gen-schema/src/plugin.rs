use liquesco_common::error::LqError;
use liquesco_processing::settings::Settings;
use liquesco_processing::code_receiver::CodeReceiver;
use liquesco_processing::plugin::Plugin;
use liquesco_processing::schema::SchemaBuilderReader;
use liquesco_processing::path::Path;
use liquesco_processing::path::Segment;
use liquesco_processing::code_receiver::Code;
use liquesco_schema::any_type::AnyType;
use liquesco_gen_html::html_writer::HtmlWriter;
use liquesco_schema::schema_builder::BuildsOwnSchema;

pub struct HtmlGenSchemaPlugin;

impl Plugin for HtmlGenSchemaPlugin {
        fn name(&self) -> &str {
            "liquesco-gen-schema"
        }
    fn description(&self) -> &str {
        "Generates HTML documentation from a liquesco schema."
    }
    fn process(&self, receiver: &mut CodeReceiver, _: &Settings) -> Result<(), LqError> {
        let mut builder = SchemaBuilderReader::default();
        let type_ref = AnyType::build_schema(&mut builder);

        let html_writer = HtmlWriter::new(&builder);
        let string = html_writer.write_to_string(type_ref)?;

        receiver.add(Path::new(Segment::new("schema.html")), 
            Code::String(string));

        Ok(())
    }
}