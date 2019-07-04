use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use liquesco_gen_doc::model::Model;
use liquesco_schema::schema_builder::DefaultSchemaBuilder;
use liquesco_schema::schema::schema_schema;
use liquesco_schema::core::TypeContainer;
use liquesco_gen_doc::{create_model, create_model_from_schema_schema};
use liquesco_gen_doc::model::card::{Card, CardId};
use liquesco_gen_doc::model_writer::CardModel;
use std::cell::RefCell;
use std::ops::Deref;

pub type ModelId = usize;
pub type MaybeCardId = JsValue;

thread_local! {
    pub static MODELS: RefCell<HashMap<ModelId, Box<dyn Model>>> = RefCell::new(HashMap::new());
}

#[wasm_bindgen]
pub fn new_schema_schema() -> ModelId {
    let model = Box::new(create_model_from_schema_schema().unwrap());
    append_model(model)
}

#[wasm_bindgen]
pub fn get_card(model : ModelId, card_id : MaybeCardId) -> JsValue {
    if !card_id.is_string() {
        JsValue::NULL
    } else {
        MODELS.with(|models| {
            let models = models.try_borrow()
                .expect("This should never fail since in a single threaded WASM environment.");
            if let Some(model) = models.get(&model) {
                if let Some(card) = model.card(
                    &CardId::new(card_id.as_string().unwrap())) {
                    JsValue::from_serde(card).unwrap()
                } else {
                    JsValue::NULL
                }
            } else {
                JsValue::NULL
            }
        })
    }
}

#[wasm_bindgen]
pub fn get_root_id(model : ModelId) -> MaybeCardId {
    MODELS.with(|models| {
        let models = models.try_borrow()
            .expect("This should never fail since in a single threaded WASM environment.");
        if let Some(model) = models.get(&model) {
                JsValue::from(model.root_id().as_str())
        } else {
            JsValue::NULL
        }
    })
}

fn append_model(model : Box<dyn Model>) -> ModelId {
    MODELS.with(|models| {
        let mut models_mut = models.try_borrow_mut()
            .expect("This should never fail since in a single threaded WASM environment.");
        let len = models_mut.len();
        let id = len;
        models_mut.insert(id, model);
        id
    })
}

