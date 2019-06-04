// TODO: Rename file to "element_writer"

use liquesco_schema::core::TypeContainer;
use crate::type_description::type_description;
use crate::usage::Usage;
use liquesco_common::error::LqError;
use liquesco_processing::type_info::TypeInfo;
use liquesco_schema::core::Type;
use liquesco_schema::core::TypeRef;
use liquesco_schema::identifier::{Format, Identifier};
use minidom::Element;
use std::borrow::Cow;

pub trait TypedElementWriter {
    type T: Type + Sized;
    fn write(ctx: &Context, typ: &Self::T) -> Result<Element, LqError>;
}

pub trait MaybeElementWriter {
    fn write(ctx: &Context) -> Result<Option<Element>, LqError>;
}

pub trait ElementWriter {
    fn write(ctx: &Context) -> Result<Element, LqError>;
}

pub struct Context<'a> {
    schema: &'a TypeContainer,
    type_info: TypeInfo<'a>,
    usage: &'a mut Usage,
}

impl<'a> Context<'a> {
    pub fn new(schema: &'a TypeContainer, type_info: TypeInfo<'a>, usage: &'a mut Usage) -> Self {
        Self {
            schema,
            type_info,
            usage,
        }
    }
}

pub trait ContextProvider<'a> {
    fn schema(&self) -> &TypeContainer;
    fn type_info(&self) -> &TypeInfo<'a>;
    fn usage(&self) -> &Usage;
    fn usage_mut(&mut self) -> &mut Usage;
}

impl<'a> ContextFunctions<'a> for Context<'a> {}

impl<'a> ContextProvider<'a> for Context<'a> {
    fn schema(&self) -> &TypeContainer {
        self.schema
    }

    fn type_info(&self) -> &TypeInfo<'a> {
        &self.type_info
    }

    fn usage(&self) -> &Usage {
        &self.usage
    }

    fn usage_mut(&mut self) -> &mut Usage {
        &mut self.usage
    }
}

impl<'a> ContextFunctions<'a> for ContextProvider<'a> {}

pub trait ContextFunctions<'a>: ContextProvider<'a> {
    fn display_name(&self) -> Cow<'static, str> {
        Self::display_name_for(&self.type_info())
    }

    fn display_name_for(type_info : &TypeInfo) -> Cow<'static, str> {
        let identifier: &Identifier = type_info.identifier();
        Cow::Owned(format!("Type[{}]", identifier.to_string(Format::SnakeCase)))
    }

    fn self_anchor_id(&self) -> Cow<'static, str> {
        anchor_id_for(&self.type_info())
    }

    fn anchor_id_for(&self, target: &TypeRef) -> Result<Cow<'static, str>, LqError> {
        let type_info = TypeInfo::try_from(self.schema(), target)?;
        Ok(anchor_id_for(&type_info))
    }

    fn link_to(&self, target: &TypeRef) -> Result<Element, LqError> {
        let type_info = TypeInfo::try_from(self.schema(), target)?;

        let anchor_id = self.anchor_id_for(target)?;
        let mut a = Element::builder("a")
            .attr("href", format!("#{target}", target = &anchor_id))
            .build();

        let name = Self::display_name_for(&type_info);
        let (type_name, _) = type_description(type_info.any_type());
        a.append_text_node(format!("{name} ({type})", name = name, type = type_name));
        Ok(a)
    }
}

fn anchor_id_for(type_info: &TypeInfo) -> Cow<'static, str> {
    Cow::Owned(type_info.identifier().to_string(Format::SnakeCase))
}
