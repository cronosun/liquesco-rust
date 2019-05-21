use crate::schema::{NameSupplier, SchemaReader};
use liquesco_common::error::LqError;
use liquesco_schema::core::TypeRef;
use liquesco_schema::identifier::Format;
use minidom::Element;
use std::collections::HashSet;

pub struct HtmlWriter<'a> {
    schema: &'a SchemaReader,
    written: HashSet<TypeRef>,
    name_supplier: NameSupplier,
    types: Vec<Element>,
}

impl<'a> HtmlWriter<'a> {
    pub fn new(schema: &'a SchemaReader) -> Self {
        Self {
            schema,
            written: HashSet::default(),
            name_supplier: NameSupplier::default(),
            types: Vec::default(),
        }
    }

    pub fn finish_to_element(self) -> Element {
        let mut body_element = Element::builder("body");

        for r#type in self.types {
            body_element = body_element.append(r#type);
        }

        let end_element = Element::builder("html").append(body_element.build());
        end_element.build()
    }

    pub fn finish_to_vec(self) -> Result<Vec<u8>, LqError> {
        let element = self.finish_to_element();
        let mut vec = Vec::<u8>::default();
        element.write_to(&mut vec).unwrap(); // TODO
        Ok(vec)
    }
}

impl<'a> HtmlWriter<'a> {
    pub fn write(&mut self, type_ref: TypeRef) {
        if self.written.contains(&type_ref) {
            return;
        }

        self.written.insert(type_ref);
        let any_type = self.schema.require(type_ref);

        let name = self.name_supplier.display_name_for(type_ref, any_type);
        let mut h1 = Element::builder("h1").build();
        h1.append_text_node(name.to_string(Format::SnakeCase));

        self.types.push(h1);
    }
}
