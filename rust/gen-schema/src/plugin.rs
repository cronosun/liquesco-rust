use liquesco_common::error::LqError;
use liquesco_gen_html::html_writer::HtmlWriter;
use liquesco_processing::code_receiver::Code;
use liquesco_processing::code_receiver::CodeReceiver;
use liquesco_processing::path::Path;
use liquesco_processing::path::Segment;
use liquesco_processing::plugin::Plugin;
use liquesco_processing::settings::Settings;
use liquesco_schema::schema::schema_schema;
use liquesco_schema::schema_builder::DefaultSchemaBuilder;
use liquesco_schema::core::TypeContainer;

pub struct HtmlGenSchemaPlugin;

impl Plugin for HtmlGenSchemaPlugin {
    fn name(&self) -> &str {
        "liquesco-gen-schema"
    }
    fn description(&self) -> &str {
        "Generates HTML documentation from a liquesco schema."
    }
    fn process(&self, receiver: &mut CodeReceiver, _: &Settings) -> Result<(), LqError> {
        let builder = DefaultSchemaBuilder::default();
        let schema = schema_schema(builder)?;
        let type_container : &TypeContainer = &schema;

        let html_writer = HtmlWriter::new(type_container);
        let string = html_writer.write_to_string(schema.root())?;

        receiver.add(Path::new(Segment::new("schema.html")), Code::String(string));

        Ok(())
    }
}
