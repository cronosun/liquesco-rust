use crate::reference::Reference;
use crate::type_description::type_description;
use crate::types::write_body;
use crate::types::BodyWriteContext;
use crate::usage::Usage;
use liquesco_common::error::LqError;
use liquesco_processing::names::Names;
use liquesco_processing::schema::SchemaReader;
use liquesco_schema::core::Type;
use liquesco_schema::metadata::WithMetadata;
use minidom::Element;

use liquesco_schema::core::TypeRef;
use liquesco_schema::identifier::Format;
use std::collections::HashMap;

pub struct HtmlWriter<'a> {
    schema: &'a SchemaReader,
    names: Names,
    bodies: HashMap<TypeRef, Element>,
    usage: Usage,
}

impl<'a> HtmlWriter<'a> {
    pub fn new(schema: &'a SchemaReader) -> Self {
        Self {
            schema,
            names: Names::default(),
            bodies: HashMap::default(),
            usage: Usage::default(),
        }
    }
}

impl HtmlWriter<'_> {
    pub fn write_to_string(mut self, type_ref: TypeRef) -> Result<String, LqError> {
        let element = self.write(type_ref);
        let mut vec = Vec::<u8>::default();
        element.write_to(&mut vec).unwrap(); // TODO
        Ok(String::from_utf8(vec).unwrap()) // TODO
    }

    fn write(&mut self, type_ref: TypeRef) -> Element {
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

    fn add_types(&mut self, target: &mut Element, type_ref: TypeRef) {
        let maybe_body = self.bodies.remove(&type_ref);
        if let Some(body) = maybe_body {
            let mut section = Element::builder("section")
                .attr("class", "liquesco-type")
                .build();

            // first the header
            section.append_child(self.type_header(type_ref));
            // now the body
            let mut body_content = Element::builder("div")
                .attr("class", "liquesco-type-body")
                .build();
            body_content.append_child(body);
            section.append_child(body_content);
            // now the footer
            if let Some(footer) = self.type_footer(type_ref) {
                section.append_child(footer);
            }

            target.append_child(section);

            // now also add all dependencies
            let dependencies = self.usage.uses(&type_ref).clone();
            for dependency in dependencies {
                self.add_types(target, dependency);
            }
        }
    }

    /// we need to process all bodies in advance (to get all
    /// dependencies correct).
    fn add_body(&mut self, type_ref: TypeRef) {
        if self.bodies.contains_key(&type_ref) {
            // no need to write
            return;
        }
        let write_context = BodyWriteContext {
            schema: self.schema,
            type_ref,
            names: &mut self.names,
            usage: &mut self.usage,
        };
        self.bodies.insert(type_ref, write_body(write_context));

        // add all embedded references
        let any_type = self.schema.require(type_ref);
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
            self.add_body(dependency);
        }
    }

    fn type_header(&mut self, type_ref: TypeRef) -> Element {
        let type_info = self.schema.type_info(type_ref);

        // name / title
        let mut header_title = Element::builder("div").attr("class", "liquesco-type-header-title");
        let name = self.names.display_name_for(&type_info);
        let mut title = Element::builder("h1").build();
        title.append_text_node(name.to_string(Format::SnakeCase));
        header_title = header_title.append(title);

        let mut header_body = Element::builder("div").attr("class", "liquesco-type-header-body");

        // type info
        let (type_name, type_description) = type_description(type_info.any_type);
        let mut type_info_elem = Element::builder("p")
            .attr("class", "liquesco-type-info")
            .build();
        let mut type_name_elem = Element::bare("em");
        type_name_elem.append_text_node(type_name);
        type_info_elem.append_child(type_name_elem);
        type_info_elem.append_text_node(": ");
        type_info_elem.append_text_node(type_description);
        header_body = header_body.append(type_info_elem);

        // description?
        if let Some(description) = type_info.any_type.meta().description() {
            let mut description_elem = Element::builder("p")
                .attr("class", "liquesco-description")
                .build();
            description_elem.append_text_node(description);
            header_body = header_body.append(description_elem);
        }

        let id = Reference {
            type_info: &type_info,
            names: &mut self.names,
        }
        .anchor_id();
        Element::builder("div")
            .attr("id", id)
            .append(header_title)
            .append(header_body.build())
            .build()
    }

    fn type_footer(&mut self, type_ref: TypeRef) -> Option<Element> {
        let used_by = self.usage.is_used_by(&type_ref).clone();
        if !used_by.is_empty() {
            let mut used_by_element = Element::builder("div")
                .attr("class", "liquesco-type-footer")
                .build();
            let mut text = Element::bare("p");
            text.append_text_node("This type is used by:");
            used_by_element.append_child(text);

            let mut ul = Element::bare("ul");
            for used_by_item in used_by {
                let type_info = self.schema.type_info(used_by_item);
                let link = Reference {
                    type_info: &type_info,
                    names: &mut self.names,
                }
                .link();
                let mut li = Element::bare("li");
                li.append_child(link);
                ul.append_child(li);
            }
            used_by_element.append_child(ul);
            Some(used_by_element)
        } else {
            None
        }
    }
}
