use crate::demo::type_info::type_info;
use crate::schema::{NameSupplier, SchemaReader};
use liquesco_common::error::LqError;
use liquesco_schema::core::TypeRef;
use liquesco_schema::identifier::Format;
use minidom::Element;
use std::collections::HashSet;

pub struct HtmlWriter<'a> {
    pub(crate) schema: &'a SchemaReader,
    written: HashSet<TypeRef>,
    pub(crate) name_supplier: NameSupplier,
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

    pub fn finish_to_element(mut self) -> Element {
        let mut head_element = Element::builder("head");
        let mut style = Element::bare("style");
        style.append_text_node(include_str!("style.css"));

        head_element = head_element.append(style);

        let mut body_element = Element::builder("body");

        self.types.reverse();
        for r#type in self.types {
            body_element = body_element.append(r#type);
        }

        let end_element = Element::builder("html")
            .append(head_element.build())
            .append(body_element.build());
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

        let mut article = Element::builder("article");
        article = article.attr("id", self.ref_anchor_id(any_type, type_ref));
        article = article.attr("class", "liquesco-type");

        // name / title
        let name = self.name_supplier.display_name_for(type_ref, any_type);
        //let technical_name = self.name_supplier.technical_name_for(&mut self.schema, type_ref);
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
        self.types.push(article.build());
    }
}
