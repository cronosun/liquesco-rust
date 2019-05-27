use crate::path::Path;
use liquesco_common::error::LqError;
use std::collections::HashMap;
use std::io::Read;

pub trait CodeReceiver {
    fn add(&mut self, path: Path, code: Code);
}

pub enum Code {
    Read(Box<dyn Read>),
    String(String),
}

pub struct DefaultCodeReceiver {
    artifacts: HashMap<Path, Code>,
}

impl Default for DefaultCodeReceiver {
    fn default() -> Self {
        Self {
            artifacts: HashMap::default(),
        }
    }
}

impl CodeReceiver for DefaultCodeReceiver {
    fn add(&mut self, path: Path, code: Code) {
        self.artifacts.insert(path, code);
    }
}

impl DefaultCodeReceiver {
    pub fn take_string(&mut self, path: &Path) -> Result<Option<String>, LqError> {
        if let Some(code) = self.artifacts.remove(path) {
            let string = match code {
                Code::String(string) => string,
                Code::Read(mut read) => {
                    let mut string = std::string::String::default();
                    read.read_to_string(&mut string).unwrap(); // TODO
                    string
                }
            };
            Ok(Some(string))
        } else {
            Ok(None)
        }
    }
}
