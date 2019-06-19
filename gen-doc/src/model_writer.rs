use crate::usage::Usage;
use liquesco_common::error::LqError;
use minidom::Element;

use liquesco_processing::type_info::TypeInfo;
use liquesco_schema::core::Type;
use liquesco_schema::core::TypeContainer;
use liquesco_schema::core::TypeRef;
use std::collections::HashMap;
use crate::model::card::{Card, Accent, CardId};
use crate::model::row::Row;
use crate::type_description::type_description;
use crate::context::{Context, ContextFunctions};
use crate::types::write_type_body;
use std::mem;
use crate::model::Model;
use std::convert::TryFrom;
use std::borrow::Cow;
use std::marker::PhantomData;
use crate::type_parts::{TypeHeader, TypeFooter};
use crate::type_writer::TypePartWriter;

pub struct ModelWriter<'a> {
    schema: &'a TypeContainer,
    bodies: HashMap<TypeRef, Vec<Row<'static>>>,
    cards : HashMap<CardId, Card<'static>>,
    usage: Usage,
}

impl<'a> ModelWriter<'a> {
    pub fn new(schema: &'a TypeContainer) -> Self {
        Self {
            schema,
            bodies: HashMap::default(),
            cards : HashMap::default(),
            usage: Usage::default(),
        }
    }
}

pub struct CardModel {
    root : CardId,
    cards : HashMap<CardId, Card<'static>>,
}

impl Model for CardModel {
    fn card(&self, id: &CardId) -> Option<&Card> {
         self.cards.get(id)
    }

    fn root(&self) -> &Card {
        if let Some(root) = self.cards.get(&self.root) {
            root
        } else {
            panic!(format!("Root card not found. This is an implementation errors. All \
            cards I have: {:?}. Root id is {:?}.", self.cards, self.root))
        }
    }

    fn root_id(&self) -> &CardId {
       &self.root
    }
}

impl<'a> ModelWriter<'a> {

    pub fn process(mut self, root_type_ref : &TypeRef) -> Result<CardModel, LqError> {
        // First add all bodies (will also compute "usage")
        self.compute_bodies_recursively(root_type_ref)?;
        self.create_cards_from_bodies(root_type_ref)?;

        Ok(CardModel {
            root : CardId::from(root_type_ref),
            cards : self.cards
        })
    }

    fn create_cards_from_bodies(&mut self, root_type_ref : &TypeRef) -> Result<(), LqError> {
        let bodies = mem::replace(&mut self.bodies, HashMap::new());

        // now create cards
        for (type_ref, mut body) in bodies.into_iter() {
            let type_info = TypeInfo::try_from(self.schema, &type_ref)?;
            let context = Context::new(self.schema, type_info.clone(), &mut self.usage);

            let (accent_num, type_name, _) = type_description(type_info.any_type());
            // accent
            let accent = Accent::new(accent_num);
            // title
            let title = context.display_name();

            let mut card = Card::new(CardId::from(&type_ref),title, accent);
            card = card.with_sub_title(type_name);

            // add header
            let mut rows = TypeHeader::write(&context)?;
            rows.append(&mut body);

            // add footer
            let mut footer_rows  = TypeFooter::write(&context)?;
            rows.append(&mut footer_rows);

            card = card.with_rows(rows);

            self.cards.insert(CardId::from(&type_ref), card);
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


