use crate::usage::Usage;
use liquesco_common::error::LqError;

use crate::context::{Context, ContextFunctions};
use crate::model::card::{Accent, Card, CardId};
use crate::model::row::Row;
use crate::model::{Model, SectionId};
use crate::type_description::type_description;
use crate::type_parts::{TypeFooter, TypeHeader};
use crate::type_writer::TypePartWriter;
use crate::types::write_type_body;
use liquesco_processing::type_info::TypeInfo;
use liquesco_schema::core::Type;
use liquesco_schema::core::TypeContainer;
use liquesco_schema::core::TypeRef;
use std::collections::HashMap;
use std::mem;
use crate::model::builder::ModelBuilder;
use std::convert::TryFrom;

pub struct ModelWriter<'a> {
    builder : ModelBuilder,
    schema: &'a TypeContainer,
    bodies: HashMap<TypeRef, Vec<Row<'static>>>,
    usage: Usage,
}

impl<'a> ModelWriter<'a> {
    pub fn new(schema: &'a TypeContainer) -> Self {
        Self {
            builder : ModelBuilder::default(),
            schema,
            bodies: HashMap::default(),
            usage: Usage::default(),
        }
    }
}

impl<'a> ModelWriter<'a> {

    pub fn process(mut self, root_type_ref: &TypeRef) -> Result<impl Model, LqError> {
        let types_section = self.builder.add_section("Types");

        // First add all bodies (will also compute "usage")
        self.compute_bodies_recursively(root_type_ref)?;
        self.create_cards_from_bodies(&types_section)?;

        self.builder.set_title("Liquesco type documentation");
        Ok(self.builder.into_model())
    }

    fn create_cards_from_bodies(&mut self, types_section : &SectionId) -> Result<(), LqError> {
        let bodies = mem::replace(&mut self.bodies, HashMap::new());

        // now create cards
        for (type_ref, mut body) in bodies.into_iter() {
            let type_info = TypeInfo::try_from(self.schema, &type_ref)?;
            let context = Context::new(self.schema, type_info.clone(), &mut self.usage);

            let (accent_num, _, _) = type_description(type_info.any_type());
            // accent
            let accent = Accent::new(accent_num);
            // title
            let title = context.display_name();

            let mut card = Card::new(CardId::try_from(&type_ref)?, title, accent);

            // add header
            let mut rows = TypeHeader::write(&context)?;
            rows.append(&mut body);

            // add footer
            let mut footer_rows = TypeFooter::write(&context)?;
            rows.append(&mut footer_rows);

            card = card.with_rows(rows);

            let card_id = CardId::try_from(&type_ref)?;
            self.builder.add_card(&card_id, card);

            // add that card to types section
            self.builder.add_to_section(types_section, &card_id)?;
        }
        Ok(())
    }

    /// First we need to process everything (this will compute dependencies iin "usage")
    fn compute_bodies_recursively(&mut self, type_ref: &TypeRef) -> Result<(), LqError> {
        if self.bodies.contains_key(&type_ref) {
            // no need to write, already processed
            return Ok(());
        }

        let type_info = TypeInfo::try_from(self.schema, &type_ref)?;
        let write_context = Context::new(self.schema, type_info, &mut self.usage);
        self.bodies
            .insert(type_ref.clone(), write_type_body(&write_context)?);

        // add all embedded references
        let any_type = self.schema.require_type(&type_ref)?;
        for index in 0..std::usize::MAX {
            let maybe_ref = any_type.reference(index);
            if let Some(reference) = maybe_ref {
                self.usage.set_uses(&type_ref, reference);
            } else {
                break;
            }
        }

        // now write all dependencies
        let dependencies = self.usage.uses(&type_ref).clone();
        for dependency in dependencies {
            self.compute_bodies_recursively(&dependency)?;
        }
        Ok(())
    }
}
