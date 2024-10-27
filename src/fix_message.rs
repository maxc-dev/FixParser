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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let message = "8=FIX.4.2|9=12|35=A|49=CLIENT|56=SERVER|10=123|";
        let fix_message = FixMessage::parse(message);

        assert_eq!(fix_message.fields.get("8"), Some(&"FIX.4.2".to_string()));
        assert_eq!(fix_message.fields.get("9"), Some(&"12".to_string()));
        assert_eq!(fix_message.fields.get("35"), Some(&"A".to_string()));
        assert_eq!(fix_message.fields.get("49"), Some(&"CLIENT".to_string()));
        assert_eq!(fix_message.fields.get("56"), Some(&"SERVER".to_string()));
        assert_eq!(fix_message.fields.get("10"), Some(&"123".to_string()));
    }

    #[test]
    fn test_get() {
        let message = "8=FIX.4.2|9=12|35=A|49=CLIENT|56=SERVER|10=123|";
        let fix_message = FixMessage::parse(message);

        assert_eq!(fix_message.get("8"), Some(&"FIX.4.2".to_string()));
        assert_eq!(fix_message.get("9"), Some(&"12".to_string()));
        assert_eq!(fix_message.get("35"), Some(&"A".to_string()));
        assert_eq!(fix_message.get("49"), Some(&"CLIENT".to_string()));
        assert_eq!(fix_message.get("56"), Some(&"SERVER".to_string()));
        assert_eq!(fix_message.get("10"), Some(&"123".to_string()));
        assert_eq!(fix_message.get("999"), None);
    }

    #[test]
    fn test_parse_empty_message() {
        let message = "";
        let fix_message = FixMessage::parse(message);

        assert!(fix_message.fields.is_empty());
    }

    #[test]
    fn test_parse_no_delimiters() {
        let message = "8=FIX.4.2";
        let fix_message = FixMessage::parse(message);

        assert_eq!(fix_message.fields.get("8"), Some(&"FIX.4.2".to_string()));
        assert_eq!(fix_message.fields.len(), 1);
    }

    #[test]
    fn test_parse_multiple_delimiters() {
        let message = "8=FIX.4.2|||9=12||35=A|";
        let fix_message = FixMessage::parse(message);

        assert_eq!(fix_message.fields.get("8"), Some(&"FIX.4.2".to_string()));
        assert_eq!(fix_message.fields.get("9"), Some(&"12".to_string()));
        assert_eq!(fix_message.fields.get("35"), Some(&"A".to_string()));
        assert_eq!(fix_message.fields.len(), 3);
    }

    #[test]
    fn test_parse_empty_fields() {
        let message = "8=|9=|35=|";
        let fix_message = FixMessage::parse(message);

        assert_eq!(fix_message.fields.get("8"), Some(&"".to_string()));
        assert_eq!(fix_message.fields.get("9"), Some(&"".to_string()));
        assert_eq!(fix_message.fields.get("35"), Some(&"".to_string()));
        assert_eq!(fix_message.fields.len(), 3);
    }

    #[test]
    fn test_get_non_existent_tag() {
        let message = "8=FIX.4.2|9=12|35=A|49=CLIENT|56=SERVER|10=123|";
        let fix_message = FixMessage::parse(message);

        assert_eq!(fix_message.get("999"), None);
    }

    #[test]
    fn test_into_iterator() {
        let message = "8=FIX.4.2|9=12|35=A|49=CLIENT|56=SERVER|10=123|";
        let fix_message = FixMessage::parse(message);
        let iter = fix_message.into_iter();
        let mut result = HashMap::new();

        for (key, value) in iter {
            result.insert(key, value);
        }

        let expected: HashMap<String, String> = [
            ("8".to_string(), "FIX.4.2".to_string()),
            ("9".to_string(), "12".to_string()),
            ("35".to_string(), "A".to_string()),
            ("49".to_string(), "CLIENT".to_string()),
            ("56".to_string(), "SERVER".to_string()),
            ("10".to_string(), "123".to_string()),
        ]
            .iter()
            .cloned()
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_into_iterator_empty_message() {
        let message = "";
        let fix_message = FixMessage::parse(message);
        let iter = fix_message.into_iter();
        let mut result = HashMap::new();

        for (key, value) in iter {
            result.insert(key, value);
        }

        let expected: HashMap<String, String> = HashMap::new();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_into_iterator_single_field() {
        let message = "8=FIX.4.2|";
        let fix_message = FixMessage::parse(message);
        let iter = fix_message.into_iter();
        let mut result = HashMap::new();

        for (key, value) in iter {
            result.insert(key, value);
        }

        let expected: HashMap<String, String> = [("8".to_string(), "FIX.4.2".to_string())]
            .iter()
            .cloned()
            .collect();

        assert_eq!(result, expected);
    }
}