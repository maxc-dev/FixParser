use crate::fix_message::FixMessage;
use crate::fix_message_parser::FixMessageParser;
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
            match FixMessageParser::parse_message(&line) {
                FixMessage::Unknown => eprintln!("Unknown message type, ignoring: {}", line),
                fix_message => messages.push(fix_message),
            }
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