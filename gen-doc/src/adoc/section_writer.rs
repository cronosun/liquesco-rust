use crate::model::{Model, SectionId};
use liquesco_processing::text::Text;
use liquesco_common::error::LqError;
use crate::adoc::card_writer::CardWriter;

pub(crate) struct SectionWriter<'a> {
    pub model: &'a Model,
    pub text: &'a mut Text,
    pub section : SectionId,
}

impl<'a> SectionWriter<'a> {
    pub(crate) fn write(&mut self) -> Result<(), LqError> {
        // header
        self.text.space();
        let title = format!("= {}", self.model.section_title(&self.section)?);
        self.text.line(title);
        self.text.space();

        for card_id in self.model.section_cards(&self.section)? {
            let card = self.model.card(card_id)?;
            let mut card_writer = CardWriter::new(self.text, card);
            card_writer.write();
        }
        Ok(())
    }
}

