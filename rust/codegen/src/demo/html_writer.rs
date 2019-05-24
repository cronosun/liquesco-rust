use crate::demo::type_info::type_info;
use crate::demo::usage::Usage;
use crate::schema::{SchemaReader};
use crate::names::Names;
use liquesco_common::error::LqError;
use liquesco_schema::core::TypeRef;
use liquesco_schema::identifier::Format;
use minidom::Element;
use std::collections::HashMap;

pub struct HtmlWriter<'a> {
    pub(crate) schema: &'a SchemaReader,
    pub(crate) name_supplier: Names,
    types: HashMap<TypeRef, Element>,
    usage: Usage,
}

impl<'a> HtmlWriter<'a> {
    pub fn new(schema: &'a SchemaReader) -> Self {
        Self {
            schema,
            name_supplier: Names::default(),
            types: HashMap::default(),
            usage: Usage::default(),
        }
    }

    pub fn finish_to_element(mut self, type_ref: TypeRef) -> Element {
        self.write(type_ref);
        self.write_uses_info();

        let mut head_element = Element::builder("head");
        let mut style = Element::bare("style");
        style.append_text_node(include_str!("style.css"));

        head_element = head_element.append(style);

        let mut body_element = Element::builder("body");

        for (_, r#type) in self.types {
            body_element = body_element.append(r#type);
        }

        let end_element = Element::builder("html")
            .append(head_element.build())
            .append(body_element.build());
        end_element.build()
    }

    pub fn finish_to_string(self, type_ref: TypeRef) -> Result<String, LqError> {
        let element = self.finish_to_element(type_ref);
        let mut vec = Vec::<u8>::default();
        element.write_to(&mut vec).unwrap(); // TODO
        Ok(String::from_utf8(vec).unwrap()) // TODO
    }

}

impl<'a> HtmlWriter<'a> {
    fn write(&mut self, type_ref: TypeRef) {
        if self.types.contains_key(&type_ref) {
            // already written, skip.
            return;
        }

        let any_type = self.schema.require(type_ref);

        let mut article = Element::builder("article");
        article = article.attr("id", self.ref_anchor_id(any_type, type_ref));
        article = article.attr("class", "liquesco-type");

        // name / title
        let name = self.name_supplier.display_name_for(type_ref, any_type);
        let mut title = Element::builder("h1").build();
        title.append_text_node(name.to_string(Format::SnakeCase));

        let mut body = Element::builder("div").attr("class", "liquesco-type-body");

        // type info
        let (type_name, type_description) = type_info(any_type);
        let mut type_info_elem = Element::builder("p")
            .attr("class", "liquesco-type-info")
            .build();
        let mut type_name_elem = Element::bare("em");
        type_name_elem.append_text_node(type_name);
        type_info_elem.append_child(type_name_elem);
        type_info_elem.append_text_node(": ");
        type_info_elem.append_text_node(type_description);
        body = body.append(type_info_elem);

        // description?
        if let Some(description) = any_type.doc().description() {
            let mut description_elem = Element::builder("p")
                .attr("class", "liquesco-description")
                .build();
            description_elem.append_text_node(description);
            body = body.append(description_elem);
        }

        // the body
        let body_element = self.type_body(any_type, type_ref);
        body = body.append(
            Element::builder("div")
                .attr("class", "liquesco-type-content")
                .append(body_element)
                .build(),
        );

        article = article.append(title).append(body);
        self.types.insert(type_ref, article.build());

        self.write_used(type_ref);
    }

    fn write_uses_info(&mut self) {
        for (type_id, element) in &mut self.types {
            let used_by = self.usage.is_used_by(type_id).clone();
            if !used_by.is_empty() {
                let mut used_by_element = Element::builder("div")
                    .attr("class", "liquesco-used-by")
                    .build();
                let mut text = Element::bare("p");
                text.append_text_node("This type is used by:");
                used_by_element.append_child(text);

                //used_by_element.append(Element::
                for used_by_item in used_by {
                    let any_type = self.schema.require(used_by_item);
                   // let link_element = self.ref_link(any_type, used_by_item);
                    used_by_element.append_text_node(format!("TODO: {:?}", used_by_item))
                }

                element.append_child(used_by_element);
            }
        }
    }

    pub(crate) fn set_uses(&mut self, myself: TypeRef, uses: TypeRef) {
        self.usage.set_uses(myself, uses)
    }

    /// Processes the things this type just used.
    fn write_used(&mut self, just_written: TypeRef) {
        let uses = self.usage.uses(&just_written);
        for r#type in uses.clone() {
            self.write(r#type.clone());
        }
    }
}
