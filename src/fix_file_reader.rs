use crate::fix_message::FixMessage;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct FixMessageFileReader;

impl FixMessageFileReader {
    pub fn read_from_file<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<FixMessage>> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut messages = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let fix_message = FixMessage::parse(&line);
            messages.push(fix_message);
        }

        Ok(messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU32, Ordering};

    static FILE_COUNTER: AtomicU32 = AtomicU32::new(0);

    fn create_temp_file(content: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let counter = FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
        path.push(format!("temp_fix_message_{}.txt", counter));
        let mut file = File::create(&path).unwrap();
        write!(file, "{}", content).unwrap();
        path
    }

    #[test]
    fn test_read_from_file() {
        let content = "8=FIX.4.2|9=12|35=A|49=CLIENT|56=SERVER|10=123|\n8=FIX.4.2|9=13|35=B|49=CLIENT|56=SERVER|10=124|";
        let file_path = create_temp_file(content);
        let messages = FixMessageFileReader::read_from_file(&file_path).unwrap();

        assert_eq!(messages.len(), 2);

        let expected_fields_1: HashMap<String, String> = [
            ("8", "FIX.4.2"),
            ("9", "12"),
            ("35", "A"),
            ("49", "CLIENT"),
            ("56", "SERVER"),
            ("10", "123"),
        ]
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        let expected_fields_2: HashMap<String, String> = [
            ("8", "FIX.4.2"),
            ("9", "13"),
            ("35", "B"),
            ("49", "CLIENT"),
            ("56", "SERVER"),
            ("10", "124"),
        ]
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        for (key, value) in &expected_fields_1 {
            assert_eq!(messages[0].get(key), Some(value));
        }

        for (key, value) in &expected_fields_2 {
            assert_eq!(messages[1].get(key), Some(value));
        }
    }

    #[test]
    fn test_read_from_non_existent_file() {
        let file_path = PathBuf::from("non_existent_file.txt");
        let result = FixMessageFileReader::read_from_file(&file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_empty_file() {
        let file_path = create_temp_file("");
        let messages = FixMessageFileReader::read_from_file(&file_path).unwrap();
        assert!(messages.is_empty());
    }
}