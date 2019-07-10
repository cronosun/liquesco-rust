use liquesco_gen_doc::adoc::AsciiDoc;
use liquesco_gen_doc::create_model;
use liquesco_processing::text::Text;
use liquesco_schema::core::TypeContainer;
use liquesco_schema::schema::schema_schema;
use liquesco_schema::schema_builder::DefaultSchemaBuilder;

#[test]
fn test_write_to_ascii_doc() {
    let builder = DefaultSchemaBuilder::default();
    let schema = schema_schema(builder).unwrap();
    let type_container: &TypeContainer = &schema;

    let model = create_model(type_container).unwrap();

    let mut text = Text::default();
    AsciiDoc::write_to(&AsciiDoc::new(), &model, &mut text).unwrap();
    let string: String = text.into();

    println!("{}", string);
}
