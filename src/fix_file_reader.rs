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