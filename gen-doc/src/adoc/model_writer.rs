use crate::adoc::card_writer::CardWriter;
use crate::model::card::CardId;
use crate::model::Model;
use liquesco_processing::text::Text;
use std::collections::HashSet;
use crate::adoc::section_writer::SectionWriter;
use liquesco_common::error::LqError;

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

    pub(crate) fn write(&mut self) -> Result<(), LqError> {
        self.write_header();

        for section_id in self.model().sections() {
            let mut section_writer = SectionWriter {
                model : self.model,
                text : self.text,
                section : section_id.clone()
            };
            section_writer.write()?;
        }
        Ok(())
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
