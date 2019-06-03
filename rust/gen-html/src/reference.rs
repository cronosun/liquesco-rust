use crate::type_description::type_description;

use liquesco_processing::type_info::TypeInfo;
use liquesco_schema::identifier::Format;
use minidom::Element;
use std::borrow::Cow;

/// TODO: Remove
pub struct Reference<'a> {
    pub type_info: &'a TypeInfo<'a>,
}

/// TODO: Remove
impl<'a> Reference<'a> {
    pub fn anchor_id(&self) -> Cow<'static,str> {
        generate_target_for(&self.type_info)
    }

    pub fn link(&mut self) -> Element {
        let anchor_id = self.anchor_id();
        let mut a = Element::builder("a")
            .attr("href", format!("#{target}", target = &anchor_id))
            .build();

        let name = self.type_info.display_name();
        let (type_name, _) = type_description(self.type_info.any_type);
        a.append_text_node(format!("{name} [{type}]", name = name, type = type_name));
        a
    }
}

/// TODO: Remove
fn generate_target_for(type_info : &TypeInfo) -> Cow<'static, str> {
    if let Some(id) = &type_info.id {
        Cow::Owned(id.to_string(Format::SnakeCase))
    } else {
        Cow::Borrowed("$THE_ROOT")
    }
}