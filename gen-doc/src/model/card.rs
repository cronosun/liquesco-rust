use std::borrow::Cow;
use crate::model::row::Row;
use core::convert::TryFrom;
use liquesco_common::error::LqError;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Card<'a> {
    id : Cow<'a, CardId>,
    title : Cow<'a, str>,
    accent : Accent,
    rows : Vec<Row<'a>>,
}

impl<'a> Card<'a> {

    pub fn new<TCardId, TTitle>(id : TCardId, title : TTitle, accent : Accent) -> Self
    where TCardId : Into<Cow<'a, CardId>>, TTitle : Into<Cow<'a, str>>{
        Self {
            id : id.into(),
            title : title.into(),
            accent,
            rows : vec![],
        }
    }

    pub fn with_rows(mut self, rows : Vec<Row<'a>>) -> Self {
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
}

/// Identifies a card uniquely.
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct CardId(String);

impl CardId {
    pub fn new<T>(string : T) -> Self  where T : Into<String>{
        Self(string.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Into<Cow<'static, CardId>> for CardId {
    fn into(self) -> Cow<'static, CardId>  {
        Cow::Owned(self)
    }
}

/// One of the 255 accents.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Accent(u8);

impl Accent {
    pub fn new(num : u8) -> Accent {
        Accent(num)
    }
}
