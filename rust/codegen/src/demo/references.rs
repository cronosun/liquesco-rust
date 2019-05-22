use liquesco_schema::any_type::AnyType;
use crate::demo::html_writer::HtmlWriter;
use crate::demo::type_info::type_info;
use liquesco_schema::core::TypeRef;
use liquesco_schema::identifier::Format;
use minidom::Element;

impl<'a> HtmlWriter<'a> {

    pub(crate) fn ref_anchor_id(&mut self, any_type : &AnyType, type_ref : TypeRef) -> String {
        let id = self.name_supplier.technical_name_for(any_type, type_ref);
        id.to_string(Format::SnakeCase)
    }

    pub(crate) fn ref_link(&mut self, any_type : &AnyType, type_ref : TypeRef) -> Element {
        let anchor_id = self.ref_anchor_id(any_type, type_ref);
        let mut a = Element::builder("a").attr("href", format!("#{target}", target = anchor_id)).build();

        let name = self.name_supplier.display_name_for(type_ref, any_type).to_string(Format::SnakeCase);
        let (type_name, _) = type_info(any_type);

        a.append_text_node(format!("{name} [{type}]", name = name, type = type_name));
        a
    }

}
