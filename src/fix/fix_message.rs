use std::collections::hash_map::IntoIter;
use std::collections::HashMap;

#[derive(Debug)]
pub struct FixMessage {
    fields: HashMap<String, String>,
}

impl FixMessage {
    pub const DELIMITER: char = '|';
    pub const ASSIGNMENT: char = '=';

    pub fn parse(message: &str) -> Self {
        let mut fields = HashMap::new();
        for pair in message.split(Self::DELIMITER) {
            if let Some((key, value)) = pair.split_once(Self::ASSIGNMENT) {
                fields.insert(key.to_string(), value.to_string());
            }
        }
        FixMessage { fields }
    }

    pub fn get(&self, tag: &str) -> Option<&String> {
        self.fields.get(tag)
    }
}

impl IntoIterator for FixMessage {
    type Item = (String, String);
    type IntoIter = IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}