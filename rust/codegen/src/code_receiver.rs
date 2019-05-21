use crate::path::Path;
use crate::CodeReceiver;
use liquesco_common::error::LqError;
use std::collections::HashMap;
use std::io::Read;

pub struct DefaultCodeReceiver {
    artifacts: HashMap<Path, Box<dyn Read>>,
}

impl Default for DefaultCodeReceiver {
    fn default() -> Self {
        Self {
            artifacts: HashMap::default(),
        }
    }
}

impl CodeReceiver for DefaultCodeReceiver {
    fn add<R>(&mut self, path: Path, read: R)
    where
        R: Read + 'static,
    {
        self.artifacts.insert(path, Box::new(read));
    }
}

impl DefaultCodeReceiver {
    pub fn take_string(&mut self, path: &Path) -> Result<Option<String>, LqError> {
        if let Some(mut reader) = self.artifacts.remove(path) {
            let mut string = String::default();
            reader.read_to_string(&mut string).unwrap(); // TODO
            Ok(Some(string))
        } else {
            Ok(None)
        }
    }
}
