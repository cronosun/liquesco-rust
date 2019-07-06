use crate::model::row::Row;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug)]
pub struct Card<'a> {
    id: CardId,
    title: Cow<'a, str>,
    accent: Accent,
    rows: Vec<Row<'a>>,
}

impl<'a> Card<'a> {
    pub fn new<TTitle>(id: CardId, title: TTitle, accent: Accent) -> Self
    where
        TTitle: Into<Cow<'a, str>>,
    {
        Self {
            id,
            title: title.into(),
            accent,
            rows: vec![],
        }
    }

    pub fn with_rows(mut self, rows: Vec<Row<'a>>) -> Self {
        self.rows = rows;
        self
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    pub fn accent(&self) -> Accent {
        self.accent
    }

    pub fn id(&self) -> &CardId {
        &self.id
    }
}

/// Identifies a card uniquely.
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct CardId {
    typ : &'static str,
    number : usize,
}

impl CardId {
    pub fn new(typ : &'static str, number: usize) -> Self {
        Self {
            typ,
            number
        }
    }

    pub fn string(&self) -> String {
        format!("{}-{}", self.typ, self.number)
    }
}

/// One of the 255 accents.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Accent(u8);

impl Accent {
    pub fn new(num: u8) -> Accent {
        Accent(num)
    }
}
