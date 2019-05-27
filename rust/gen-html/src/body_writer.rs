use crate::reference::Reference;
use liquesco_processing::names::Names;
use liquesco_processing::schema::SchemaReader;
use liquesco_schema::core::Type;
use liquesco_schema::core::TypeRef;
use minidom::Element;

pub trait BodyWriter {
    type T: Type + Sized;
    fn write(ctx: &mut Context<Self::T>) -> Element;
}

pub struct Context<'a, T> {
    pub schema: &'a SchemaReader,
    pub r#type: &'a T,
    pub type_ref: TypeRef,
    pub names: &'a mut Names,
}

impl<'a, T> Context<'a, T> {
    pub fn link(&mut self, target: TypeRef) -> Element {
        let type_info = self.schema.type_info(target);
        Reference {
            type_info: &type_info,
            names: self.names,
        }
        .link()
    }
}
