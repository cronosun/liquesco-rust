use std::collections::HashMap;

pub struct Settings<'a>(HashMap<&'a str, &'a str>);

impl<'a> Settings<'a> {
    pub fn get(&self, key : &str) -> Option<&str> {
        if let Some(value) = self.0.get(key) {
            Some(*value)
        } else {
            None
        }
    }
}