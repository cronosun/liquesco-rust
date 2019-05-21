use crate::schema::{SchemaReader, NameSupplier};
use crate::text::Text;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::TypeRef;
use liquesco_schema::identifier::Format;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use minidom::{Element, Node};

pub struct HtmlWriter<'a> {
    schema: &'a SchemaReader,
    written: HashSet<TypeRef>,
    name_supplier : NameSupplier,
    types : Vec<Element>
}

impl<'a> HtmlWriter<'a> {
    pub fn new(schema : &'a SchemaReader) -> Self {
        Self {
            schema,
            written : HashSet::default(),
            name_supplier : NameSupplier::default(),
            types : Vec::default()
        }
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
        self.push(
            Element::builder("h1").append(Node::Text(name.to_string(Format::SnakeCase))).build()
        );
    }
}
