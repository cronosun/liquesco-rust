use liquesco_processing::schema::SchemaReader;
use liquesco_schema::core::Type;
use liquesco_schema::core::TypeRef;
use minidom::Element;
use liquesco_schema::identifier::{Identifier, Format};
use liquesco_common::error::LqError;
use liquesco_processing::type_info::TypeInfo;
use std::borrow::Cow;
use std::any::TypeId;
use crate::type_description::type_description;
use crate::usage::Usage;

const ROOT_ANCHOR_ID : &str = "$THE_ROOT";

/// TODO: Rename to ElementWriter
pub trait BodyWriter<'a> {
    type T: Type + Sized;
    /// TODO: Context really mut? -> Ja, wegen "usage"
    fn write(ctx: &mut Context<Self::T>) -> Result<Element, LqError>; // TODO: Yes, a result
}

pub trait MaybeElementWriter<'a> {
    type T: Type + Sized;
    fn write(ctx: &mut Context<Self::T>) -> Result<Option<Element>, LqError>;
}

pub struct Context<'a, T> {
    schema: &'a SchemaReader,
    pub type_info: TypeInfo<'a>, // TODO: private
    r#type: &'a T,
    pub usage: Usage, // TODO: Private
}

impl<'a, T> Context<'a, T> {

    pub fn schema(&self) -> &SchemaReader {
        self.schema
    }

    pub fn type_info(&self) -> &TypeInfo<'a> {
        &self.type_info
    }

    pub fn r#type(&self) -> &T {
        self.r#type
    }

    pub fn display_name(&self) -> Cow<'static, str> {
        if let Some(id) = &self.type_info.id {
            let identifier: &Identifier = id;
            Cow::Owned(format!("Type[{}]", identifier.to_string(Format::SnakeCase)))
        } else {
            Cow::Borrowed("Type[ROOT]")
        }
    }

    pub fn self_anchor_id(&self) -> Cow<'static,str> {
        anchor_id_for(Some(&self.type_info))
    }

    pub fn anchor_id_for(&self, target : Option<&TypeRef>) -> Result<Cow<'static,str>, LqError> {
        if let Some(target) = target {
            let type_info = self.schema.non_root_type_info(target)?;
            Ok(anchor_id_for(Some(&type_info)))
        } else {
            Ok(anchor_id_for(None))
        }
    }

    pub fn link_to(&self, target : Option<&TypeRef>) -> Result<Element, LqError> {
        let type_info = self.schema.type_info(target)?;

        let anchor_id = self.anchor_id();
        let mut a = Element::builder("a")
            .attr("href", format!("#{target}", target = &anchor_id))
            .build();

        let name = self.type_info.display_name();
        let (type_name, _) = type_description(type_info.any_type);
        a.append_text_node(format!("{name} [{type}]", name = name, type = type_name));
        Ok(a)
    }
}

fn anchor_id_for(type_info : Option<&TypeInfo>) -> Cow<'static,str> {
    if let Some(type_info) = type_info {
        if let Some(id) = &type_info.id {
            let identifier: &Identifier = id;
            Cow::Owned(identifier.to_string(Format::SnakeCase))
        } else {
            Cow::Borrowed(ROOT_ANCHOR_ID)
        }
    } else {
        Cow::Borrowed(ROOT_ANCHOR_ID)
    }
}