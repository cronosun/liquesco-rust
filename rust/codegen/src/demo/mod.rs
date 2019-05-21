pub mod md_writer;

use crate::settings::Settings;
use crate::CodeReceiver;
use crate::Plugin;
use liquesco_common::error::LqError;

pub struct MdCodeGen;

impl Plugin for MdCodeGen {
    fn name(&self) -> &str {
        "schema-md-gen"
    }

    fn description(&self) -> &str {
        "Generates Markdown documentation for the liquesco schema language."
    }

    fn process<CR>(&self, receiver: &mut &CR, settings: &Settings) -> Result<(), LqError>
    where
        CR: CodeReceiver,
    {
            Ok(())
    }
}
