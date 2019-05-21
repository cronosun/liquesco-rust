use liquesco_common::error::LqError;
use std::io::Read;
use crate::path::Path;
use crate::settings::Settings;
use std::io::Write;

#[macro_use]
extern crate derive_more;

pub mod demo;
pub mod path;
pub mod settings;
pub mod text;
pub mod vec_read;
pub mod schema;

pub trait CodeReceiver {
    fn add<R>(&mut self, path: Path, read: R)
    where
        R: Read;
}

pub trait Input {
    type R : Read;
    fn get(&self, path : &Path) -> Option<Self::R>;
}

pub trait Plugin {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn process<CR>(&self, receiver: &mut &CR, settings: &Settings) -> Result<(), LqError>
    where
        CR: CodeReceiver;
}
