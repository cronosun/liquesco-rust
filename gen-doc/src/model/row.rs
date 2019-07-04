use crate::model::card::CardId;
use std::borrow::Cow;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Row<'a> {
    Association(Association<'a>),
    Section(Cow<'a, str>),
    Prim(Primitive<'a>),
    Note(Cow<'a, str>),
}

impl<'a> Row<'a> {
    pub fn text_with_link<TText, TLink>(text: TText, link: TLink) -> Self
        where TText: Into<Cow<'a, str>>, TLink: Into<Cow<'a, CardId>>  {
        Row::Prim(Primitive::text_with_link(text, link))
    }

    pub fn association_with_text<TKey, TValue>(key : TKey, value : TValue) -> Self
        where TKey : Into<Cow<'a, str>>, TValue:Into<Cow<'a, str>> {
        Row::Association(Association {
            key : key.into(),
            value : vec![Primitive::text(value)]
        })
    }

    pub fn association_with_link<TKey, TValueText, TValueLink>(
        key : TKey, value_text : TValueText, value_link : TValueLink) -> Self
        where TKey : Into<Cow<'a, str>>, TValueText:Into<Cow<'a, str>>,
              TValueLink : Into<Cow<'a, CardId>> {
        Row::Association(Association {
            key : key.into(),
            value : vec![Primitive::text_with_link(value_text, value_link)]
        })
    }

    pub fn association(association : Association<'a>) -> Self {
        Row::Association(association)
    }

    pub fn note<TValue>(value : TValue) -> Self
        where TValue:Into<Cow<'a, str>> {
        Row::Note(value.into())
    }

    pub fn section<TValue>(value : TValue) -> Self
        where TValue:Into<Cow<'a, str>> {
        Row::Section(value.into())
    }

    pub fn text<TValue>(value : TValue) -> Self
        where TValue:Into<Cow<'a, str>> {
        Row::Prim(Primitive::text(value))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Association<'a> {
    key: Cow<'a, str>,
    value: Vec<Primitive<'a>>,
}

impl<'a> Association<'a> {

    pub fn new<TKey>(key : TKey) -> Self where TKey : Into<Cow<'a, str>> {
        Self {
            key : key.into(),
            value : Vec::new()
        }
    }

    pub fn push_value(&mut self, primitive : Primitive<'a>) {
        self.value.push(primitive);
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &[Primitive] {
        &self.value
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Primitive<'a> {
    Text(Cow<'a, str>),
    TextWithLink(TextWithLink<'a>),
}

impl<'a> Primitive<'a> {
    pub fn text_with_link<TText, TLink>(text: TText, link: TLink) -> Self
        where TText: Into<Cow<'a, str>>, TLink: Into<Cow<'a, CardId>> {
        Primitive::TextWithLink(TextWithLink::new(text, link))
    }

    pub fn text<TText>(text : TText) -> Self where TText : Into<Cow<'a, str>> {
        Primitive::Text(text.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextWithLink<'a> {
    text: Cow<'a, str>,
    link: Cow<'a, CardId>,
}

impl<'a> TextWithLink<'a> {
    pub fn new<TText, TLink>(text: TText, link: TLink) -> Self
        where TText: Into<Cow<'a, str>>, TLink: Into<Cow<'a, CardId>> {
        Self {
            text: text.into(),
            link: link.into(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn link(&self) -> &CardId {
        &self.link
    }
}
