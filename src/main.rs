use std::collections::HashMap;

#[derive(Debug)]
struct FixMessage {
    fields: HashMap<String, String>,
}

impl FixMessage {
    fn parse(message: &str) -> Self {
        let mut fields = HashMap::new();
        for pair in message.split('\x01') {
            if let Some((key, value)) = pair.split_once('=') {
                fields.insert(key.to_string(), value.to_string());
            }
        }
        FixMessage { fields }
    }

    fn get(&self, tag: &str) -> Option<&String> {
        self.fields.get(tag)
    }
}

fn main() {
    let message = "8=FIX.4.2\x019=12\x0135=A\x0149=CLIENT\x0156=SERVER\x0110=123\x01";
    let fix_message = FixMessage::parse(message);
    println!("{:?}", fix_message);

    for i in vec![8, 9, 35, 49, 56, 10] {
        match fix_message.get(&i.to_string()) {
            Some(value) => println!("Field {}: {}", i, value),
            None => println!("Field {} not found", i),
        }
    }
}