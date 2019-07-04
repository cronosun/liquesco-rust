use crate::model::Model;
use liquesco_processing::text::Text;
use crate::model::card::CardId;
use std::collections::HashSet;
use crate::adoc::card_writer::CardWriter;

pub(crate) struct ModelWriter<'a> {
    model: &'a Model,
    text: &'a mut Text,
    cards_to_go: Vec<CardId>,
    written_cards: HashSet<CardId>,
}

impl<'a> ModelWriter<'a> {
    pub(crate) fn new(model: &'a Model, text: &'a mut Text) -> Self {
        Self {
            model,
            text,
            cards_to_go: Vec::new(),
            written_cards: HashSet::new(),
        }
    }

    pub(crate) fn write(&mut self) {
        self.write_header();
        self.cards_to_go.push(self.model().root_id().clone());
        while self.write_next_card() {}
    }

    fn write_next_card(&mut self) -> bool {
        if self.cards_to_go.is_empty() {
            return false;
        }

        let card_id = self.cards_to_go.remove(0);
        if let Some(card) = self.model.card(&card_id) {
            let mut card_writer = CardWriter::new(self.text, card);
            card_writer.write();
            self.written_cards.insert(card_id);

            // process dependencies
            for dependency in card_writer.take_dependencies() {
                if !self.written_cards.contains(dependency) {
                    self.cards_to_go.push(dependency.clone());
                }
            }
        } else {
            self.text().space();
            self.text().line(format!("WARNING: Card {} not found", card_id.as_str()));
            self.text().space();
        }

        // continue
        true
    }

    fn write_header(&mut self) {
        let title = self.model().title();
        let title_line = format!("= {}", title);
        self.text().line(title_line);
        self.text().line("Liquesco document generator");
        self.text().line(":doctype: article");
        self.text().line(":encoding: utf-8");
        self.text().line(":lang: en");
        self.text().line(":toc: left");
        self.text().line(":numbered:");
        self.text().space();
    }

    fn model(&self) -> &'a Model {
        self.model
    }

    fn text(&mut self) -> &mut Text {
        &mut self.text
    }
}
