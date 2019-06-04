use crate::types::write_body;
use crate::usage::Usage;
use liquesco_common::error::LqError;
use liquesco_processing::schema::SchemaReader;
use liquesco_schema::core::Type;
use minidom::Element;

use crate::body_writer::Context;
use crate::body_writer::ElementWriter;
use crate::body_writer::MaybeElementWriter;
use crate::type_components::TypeFooter;
use crate::type_components::TypeHeader;
use liquesco_schema::core::TypeRef;
use std::collections::HashMap;

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
        let element = self.write(type_ref)?;
        let mut vec = Vec::<u8>::default();
        element.write_to(&mut vec).unwrap(); // TODO
        Ok(String::from_utf8(vec).unwrap()) // TODO
    }

    fn write(&mut self, type_ref: &TypeRef) -> Result<Element, LqError> {
        let mut head_element = Element::builder("head");
        let mut style = Element::bare("style");
        style.append_text_node(include_str!("style.css"));

        head_element = head_element.append(style);

        let mut body_element = Element::builder("body").build();

        // add all types (this will also compute dependencies)
        self.add_bodies_recursively(type_ref)?;
        // now add all types to the body
        self.add_types_recursively(&mut body_element, type_ref)?;

        let end_element = Element::builder("html")
            .append(head_element.build())
            .append(body_element);
        Ok(end_element.build())
    }

    fn add_types_recursively(
        &mut self,
        target: &mut Element,
        type_ref: &TypeRef,
    ) -> Result<(), LqError> {
        let maybe_body = self.bodies.remove(&type_ref);
        if let Some(body) = maybe_body {
            let mut section = Element::builder("section")
                .attr("class", "liquesco-type")
                .build();

            let type_info = self.schema.type_info(type_ref)?;
            // Context
            let context = Context::new(self.schema, type_info, &mut self.usage);

            // first the header
            section.append_child(TypeHeader::write(&context)?);

            // now the body
            let mut body_content = Element::builder("div")
                .attr("class", "liquesco-type-body")
                .build();
            body_content.append_child(body);
            section.append_child(body_content);

            // now the footer
            if let Some(footer) = TypeFooter::write(&context)? {
                section.append_child(footer);
            }

            target.append_child(section);

            // now also add all dependencies
            let dependencies = self.usage.uses(&type_ref).clone();
            for dependency in dependencies {
                self.add_types_recursively(target, &dependency)?;
            }
        }

        Ok(())
    }

    /// we need to process all bodies in advance (to get all
    /// dependencies correct).
    fn add_bodies_recursively(&mut self, type_ref: &TypeRef) -> Result<(), LqError> {
        if self.bodies.contains_key(type_ref) {
            // no need to write
            return Ok(());
        }

        let type_info = self.schema.type_info(type_ref)?;
        let write_context = Context::new(self.schema, type_info, &mut self.usage);
        self.bodies
            .insert(type_ref.clone(), write_body(&write_context)?);

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
        let dependencies = self.usage.uses(&type_ref).clone();
        for dependency in dependencies {
            self.add_bodies_recursively(&dependency)?;
        }
        Ok(())
    }
}
