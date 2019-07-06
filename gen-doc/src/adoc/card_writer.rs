use crate::model::card::{Card, CardId};
use crate::model::row::{Association, Link, Primitive, Row};
use liquesco_processing::text::Text;
use std::collections::HashSet;

static TABLE_LINE: &str = "|====================";

pub(crate) struct CardWriter<'a> {
    text: &'a mut Text,
    in_association: bool,
    card: &'a Card<'a>,
    dependencies: HashSet<&'a CardId>,
}

impl<'a> CardWriter<'a> {
    pub(crate) fn new(text: &'a mut Text, card: &'a Card<'a>) -> Self {
        Self {
            text,
            in_association: false,
            card,
            dependencies: HashSet::new(),
        }
    }

    pub(crate) fn take_dependencies(self) -> HashSet<&'a CardId> {
        self.dependencies
    }

    pub(crate) fn write(&mut self) {
        self.write_card_header();
        for row in self.card.rows() {
            self.write_row(row);
        }
        self.maybe_end_association();
    }

    fn text(&mut self) -> &mut Text {
        self.text
    }

    fn write_card_header(&mut self) {
        self.text().space();
        // anchor
        let link_id = Self::generate_link_id(self.card.id());
        let anchor_line = format!("[[{}]]", link_id);
        self.text().line(anchor_line);
        // title
        let title_line = format!("== {}", self.card.title());
        self.text().line(title_line);
    }

    fn write_row(&mut self, row: &'a Row) {
        match row {
            Row::Note(note) => {
                self.maybe_end_association();
                self.row_write_note(&note);
            }
            Row::Section(section) => {
                self.maybe_end_association();
                self.row_write_section(&section);
            }
            Row::Prim(primitive) => {
                self.maybe_end_association();
                match primitive {
                    Primitive::Text(text) => {
                        self.row_write_text(text);
                    }
                    Primitive::Code(text) => {
                        self.row_write_code(text);
                    }
                    Primitive::Link(text_and_link) => {
                        self.row_write_link(text_and_link);
                    }
                }
            }
            Row::Association(association) => {
                self.maybe_start_association();
                self.write_association(association);
            }
        }
    }

    fn maybe_start_association(&mut self) {
        if !self.in_association {
            self.text().space();
            self.text().line("[width=\"100%\"]");
            self.text().line(TABLE_LINE);
            self.in_association = true;
        }
    }

    fn maybe_end_association(&mut self) {
        if self.in_association {
            self.text().line(TABLE_LINE);
            self.text().space();
            self.in_association = false;
        }
    }

    fn write_association(&mut self, association: &'a Association) {
        let number_of_values = association.value().len();
        match number_of_values {
            0 | 1 => {
                self.text().line(format!("| {}", association.key()));
                self.text().add(" | ");
                if let Some(value) = association.value().first() {
                    self.write_primitive(value);
                }
            }
            _ => {
                // row span
                self.text()
                    .line(format!(".+{}+| {}", number_of_values, association.key()));
                self.text().line("| ");
                for value in association.value() {
                    self.write_primitive(value);
                }
            }
        }

        self.text.new_line();
    }

    fn write_primitive(&mut self, primitive: &'a Primitive) {
        match primitive {
            Primitive::Text(text) => {
                self.text().add(&text);
            }
            Primitive::Code(text) => {
                self.text().add(format!("`{}`", &text));
            }
            Primitive::Link(text_link) => {
                self.write_link(text_link);
            }
        }
    }

    fn row_write_section(&mut self, section: &str) {
        self.text().space();
        self.text().line("*");
        self.text().add(section);
        self.text().add("*");
        self.text().space();
    }

    fn row_write_note(&mut self, note: &str) {
        self.text().space();
        self.text().line("[NOTE]");
        self.text().line("====");
        self.text().line(note);
        self.text().line("====");
        self.text().space();
    }

    fn row_write_text(&mut self, text: &str) {
        self.text().space();
        self.text().line(text);
        self.text().space();
    }

    fn row_write_code(&mut self, text: &str) {
        self.text().space();
        self.text().line("[literal]");
        self.text().line("....");
        self.text().line(text);
        self.text().line("....");
        self.text().space();
    }

    fn row_write_link(&mut self, text_with_link: &'a Link) {
        self.text().space();
        self.write_link(text_with_link);
        self.text().space();
    }

    fn write_link(&mut self, text_with_link: &'a Link) {
        self.text().add(format!(
            "xref:{}[{}]",
            Self::generate_link_id(&text_with_link.target()),
            text_with_link.text()
        ));
        self.add_dependency(text_with_link.target());
    }

    fn add_dependency(&mut self, card_id: &'a CardId) {
        self.dependencies.insert(card_id);
    }

    fn generate_link_id(id: &CardId) -> String {
        format!("card-{}", id.string())
    }
}
