use std::borrow::Cow;
use crate::model::{SectionId, Model};
use crate::model::card::{CardId, Card};
use std::collections::HashMap;
use liquesco_common::error::LqError;

pub struct ModelBuilder {
    sections : Vec<SectionId>,
    sections_map : HashMap<SectionId, Section>,
    title: Cow<'static, str>,
    cards: HashMap<CardId, Card<'static>>,
}

impl Default for ModelBuilder {
    fn default() -> Self {
    Self {
        sections : Vec::default(),
        sections_map : HashMap::default(),
        title : Cow::Borrowed(""),
        cards : HashMap::default()
    }
    }
}

struct Section {
    title: Cow<'static, str>,
    cards : Vec<CardId>
}

impl ModelBuilder {

    pub fn set_title<T>(&mut self, title : T) where T : Into<Cow<'static, str>> {
        self.title = title.into();
    }

    pub fn add_section<T>(&mut self, title : T) -> SectionId where T : Into<Cow<'static, str>> {
        let id = SectionId::new(self.sections.len());
        self.sections.push(id);
        self.sections_map.insert(id, Section {
            title : title.into(),
            cards : Vec::default()
        });
        id
    }

    /// Section must exist and the card must have been added before.
    pub fn add_to_section(&mut self, section : &SectionId, card : &CardId) -> Result<(), LqError> {
        if let Some(section) = self.sections_map.get_mut(section) {
            if self.cards.contains_key(card) {
                section.cards.push(card.clone());
                Ok(())
            } else {
                Err(LqError::new(format!("Card {:?} not found (add card first)", card)))
            }
        } else {
            Err(LqError::new(format!("Section {:?} not found (add section first)", section)))
        }
    }

    pub fn add_card(&mut self, id : &CardId, card : Card<'static>) {
        self.cards.insert(id.clone(), card);
    }

    pub fn has_card(&self, card : &CardId) -> bool {
        self.cards.contains_key(card)
    }

    pub fn into_model(self) -> impl Model {
        BuiltModel {
            sections : self.sections,
            sections_map : self.sections_map,
            title : self.title,
            cards : self.cards,
        }
    }
}

struct BuiltModel {
    sections : Vec<SectionId>,
    sections_map : HashMap<SectionId, Section>,
    title: Cow<'static, str>,
    cards: HashMap<CardId, Card<'static>>,
}

impl Model for BuiltModel {
    fn sections(&self) -> &[SectionId] {
        &self.sections
    }

    fn section_title(&self, section: &SectionId) -> Result<&str, LqError> {
        self.sections_map.get(section).ok_or_else(|| {
            LqError::new(format!("Section {:?} not found", section))
        }).map(|cow| {
            let string : &str = &cow.title;
            string
        })
    }

    fn section_cards(&self, section: &SectionId) -> Result<&[CardId], LqError> {
        if let Some(section) = self.sections_map.get(section) {
            Ok(&section.cards)
        } else {
            Err(LqError::new(format!("Section {:?} not found", section)))
        }
    }

    fn card(&self, id: &CardId) -> Result<&Card, LqError> {
        self.cards.get(id).ok_or_else(|| {
            LqError::new(format!("Card {:?} not found", id))
        })
    }

    fn title(&self) -> &str {
        &self.title
    }
}