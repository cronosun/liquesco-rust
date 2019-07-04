use liquesco_schema::core::{TypeContainer, TypeRef};
use liquesco_processing::type_info::TypeInfo;
use crate::usage::Usage;
use std::borrow::Cow;
use liquesco_schema::identifier::{Identifier, Format};
use liquesco_common::error::LqError;
use crate::model::card::CardId;
use crate::model::row::{Row, Primitive};

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

pub trait ContextProvider<'a> : ContextFunctions<'a> {
    fn schema(&self) -> &TypeContainer;
    fn type_info(&self) -> &TypeInfo<'a>;
    fn usage(&self) -> &Usage;
    fn usage_mut(&mut self) -> &mut Usage;
}

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

impl<'a> ContextFunctions<'a> for Context<'a> {
    fn display_name(&self) -> String {
        Self::display_name_for(self,&self.type_info())
    }

    fn named_link_to_type<TName>(&self, name : TName, typ : &TypeRef)
                                     -> Result<Row<'static>, LqError>
        where TName : Into<Cow<'static, str>> {
        let type_info = TypeInfo::try_from(self.schema(), typ)?;
        Ok(Row::association_with_link(
            name,
            self.display_name_for(&type_info),
            CardId::from(typ)
        ))
    }

    fn link_to_type(&self, typ: &TypeRef) -> Result<Primitive<'static>, LqError> {
        let type_info = TypeInfo::try_from(self.schema(), typ)?;
        Ok(Primitive::text_with_link(self.display_name_for(&type_info),
                                  CardId::from(typ)))
    }
}

pub trait ContextFunctions<'a> {
    fn display_name(&self) -> String;

    fn display_name_for(&self, type_info: &TypeInfo) -> String {
        let identifier: &Identifier = type_info.identifier();
        format!("Type({})", identifier.to_string(Format::SnakeCase))
    }

    fn named_link_to_type<TName>(&self, name : TName, typ : &TypeRef)
    -> Result<Row<'static>, LqError>
        where TName : Into<Cow<'static, str>>;

    fn link_to_type(&self, typ : &TypeRef) -> Result<Primitive<'static>, LqError>;
}


