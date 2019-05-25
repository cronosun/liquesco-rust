use crate::code_receiver::CodeReceiver;
use crate::path::Path;
use crate::settings::Settings;
use liquesco_common::error::LqError;
use std::io::Read;

pub trait Input {
    type R: Read;
    fn get(&self, path: &Path) -> Option<Self::R>;
}

pub trait Plugin {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn process(&self, receiver: &mut CodeReceiver, settings: &Settings) -> Result<(), LqError>;
}