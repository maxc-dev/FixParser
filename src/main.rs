mod fix {
    pub mod fix_message;
}

use fix::fix_message::FixMessage;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct FixMessageFileReader;

impl FixMessageFileReader {
    fn read_from_file<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<FixMessage>> {
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

fn main() {
    let file_path = "resources/fix_sample_message1.txt";
    match FixMessageFileReader::read_from_file(file_path) {
        Ok(messages) => {
            for fix_message in messages {
                println!("{:?}", fix_message);
                for (key, value) in fix_message {
                    println!("{}: {}", key, value);
                }
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}