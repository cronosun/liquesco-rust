use crate::type_description::type_description;

use liquesco_processing::names::Names;
use liquesco_processing::type_info::TypeInfo;
use liquesco_schema::identifier::Format;
use minidom::Element;

pub struct Reference<'a> {
    pub type_info: &'a TypeInfo<'a>,
    pub names: &'a mut Names,
}

impl<'a> Reference<'a> {
    pub fn anchor_id(&mut self) -> String {
        let id = self.names.technical_name_for(self.type_info);
        id.to_string(Format::SnakeCase)
    }

    pub fn link(&mut self) -> Element {
        let anchor_id = self.anchor_id();
        let mut a = Element::builder("a")
            .attr("href", format!("#{target}", target = anchor_id))
            .build();

        let name = self
            .names
            .display_name_for(self.type_info)
            .to_string(Format::SnakeCase);
        let (type_name, _) = type_description(self.type_info.any_type);

        a.append_text_node(format!("{name} [{type}]", name = name, type = type_name));
        a
    }
}
