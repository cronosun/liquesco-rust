use crate::type_description::type_description;
use crate::types::write_body;
use crate::types::BodyWriteContext;
use crate::usage::Usage;
use liquesco_common::error::LqError;
use liquesco_processing::schema::SchemaReader;
use liquesco_schema::core::Type;
use liquesco_schema::metadata::WithMetadata;
use minidom::Element;

use liquesco_schema::core::TypeRef;
use liquesco_schema::identifier::Format;
use std::collections::HashMap;
use liquesco_processing::type_info::TypeInfo;

pub struct HtmlWriter<'a> {
    schema: &'a SchemaReader,
    bodies: HashMap<TypeRef, Element>,
    usage: Usage,
}

impl<'a> HtmlWriter<'a> {
    pub fn new(schema: &'a SchemaReader) -> Self {
        Self {
            schema,
            bodies: HashMap::default(),
            usage: Usage::default(),
        }
    }
}

impl HtmlWriter<'_> {
    pub fn write_to_string(mut self, type_ref: &TypeRef) -> Result<String, LqError> {
        let element = self.write(type_ref);
        let mut vec = Vec::<u8>::default();
        element.write_to(&mut vec).unwrap(); // TODO
        Ok(String::from_utf8(vec).unwrap()) // TODO
    }

    fn write(&mut self, type_ref: &TypeRef) -> Element {
        let mut head_element = Element::builder("head");
        let mut style = Element::bare("style");
        style.append_text_node(include_str!("style.css"));

        head_element = head_element.append(style);

        let mut body_element = Element::builder("body").build();

        // add all types (this will also compute dependencies)
        self.add_body(type_ref);
        // now add all types to the body
        self.add_types(&mut body_element, type_ref);

        let end_element = Element::builder("html")
            .append(head_element.build())
            .append(body_element);
        end_element.build()
    }

    fn add_types(&mut self, target: &mut Element, type_ref: &TypeRef) {
        let maybe_body = self.bodies.remove(&type_ref);
        if let Some(body) = maybe_body {
            let mut section = Element::builder("section")
                .attr("class", "liquesco-type")
                .build();

            // first the header
            section.append_child(self.type_header(&type_ref));
            // now the body
            let mut body_content = Element::builder("div")
                .attr("class", "liquesco-type-body")
                .build();
            body_content.append_child(body);
            section.append_child(body_content);
            // now the footer
            if let Some(footer) = self.type_footer(&type_ref) {
                section.append_child(footer);
            }

            target.append_child(section);

            // now also add all dependencies
            let dependencies = self.usage.uses(&type_ref);
            for dependency in dependencies {
                self.add_types(target, dependency);
            }
        }
    }

    /// we need to process all bodies in advance (to get all
    /// dependencies correct).
    fn add_body(&mut self, type_ref: &TypeRef) -> Result<(), LqError> {
        if self.bodies.contains_key(type_ref) {
            // no need to write
            return Ok(());
        }
        let write_context = BodyWriteContext {
            schema: self.schema,
            type_ref : type_ref.clone(),
            usage: &mut self.usage,
        };
        self.bodies.insert(type_ref.clone(), write_body(write_context));

        // add all embedded references
        let any_type = self.schema.require_type(type_ref)?;
        for index in 0..std::usize::MAX {
            let maybe_ref = any_type.reference(index);
            if let Some(reference) = maybe_ref {
                self.usage.set_uses(type_ref, reference);
            } else {
                break;
            }
        }

        // now write all dependencies
        let dependencies = self.usage.uses(&type_ref);
        for dependency in dependencies {
            self.add_body(dependency);
        }
        Ok(())
    }

    fn type_header(&mut self, type_info: &TypeInfo) -> Element {
        // name / title
        unimplemented!()
    }

    fn type_footer(&mut self, type_ref: &TypeRef) -> Option<Element> {
       unimplemented!()
    }
}
