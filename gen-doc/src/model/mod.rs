pub mod card;
pub mod row;
pub mod builder;
use crate::model::card::Card;
use crate::model::card::CardId;
use liquesco_common::error::LqError;

pub trait Model {
    fn sections(&self) -> &[SectionId];
    fn section_title(&self, section : &SectionId) -> Result<&str, LqError>;
    fn section_cards(&self, section : &SectionId) -> Result<&[CardId], LqError>;

    fn card(&self,id: &CardId) -> Result<&Card, LqError>;

    /// Title of the model.
    fn title(&self) -> &str;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct SectionId(usize);

impl SectionId {
    pub(crate) fn new(id : usize) -> Self {
        Self(id)
    }
}