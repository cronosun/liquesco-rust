pub mod card;
pub mod row;
use crate::model::card::CardId;
use crate::model::card::Card;

pub trait Model {
    fn card(&self, id : &CardId) -> Option<&Card>;
    fn root(&self) -> &Card;
    fn root_id(&self) -> &CardId;

    /// Title of the model.
    fn title(&self) -> &str;
}