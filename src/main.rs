mod fix_file_reader;
mod fix_message;

use fix_file_reader::FixMessageFileReader;

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