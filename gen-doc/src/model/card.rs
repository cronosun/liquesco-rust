use std::borrow::Cow;
use crate::model::row::Row;
use core::convert::TryFrom;
use liquesco_common::error::LqError;

#[derive(Debug)]
pub struct Card<'a> {
    pub id : Cow<'a, CardId>,
    pub title : Cow<'a, str>,
    pub sub_title : Option<Cow<'a, str>>,
    pub accent : Accent,
    pub rows : Vec<Row<'a>>,
}

impl<'a> Card<'a> {

    pub fn new<TCardId, TTitle>(id : TCardId, title : TTitle, accent : Accent) -> Self
    where TCardId : Into<Cow<'a, CardId>>, TTitle : Into<Cow<'a, str>>{
        Self {
            id : id.into(),
            title : title.into(),
            sub_title : None,
            accent,
            rows : vec![],
        }
    }

    pub fn with_sub_title<TSubTitle>(mut self, sub_title : TSubTitle) -> Self
    where TSubTitle : Into<Cow<'a, str>>{
        self.sub_title = Some(sub_title.into());
        self
    }

    pub fn with_rows(mut self, rows : Vec<Row<'a>>) -> Self {
        self.rows = rows;
        self
    }
}

/// Identifies a card uniquely.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
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
#[derive(Debug)]
pub struct Accent(u8);

impl Accent {
    pub fn new(num : u8) -> Accent {
        Accent(num)
    }
}
