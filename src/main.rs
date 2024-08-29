use std::io;

struct Counter {
    value: i64,
}

enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}

fn main() {
    let mut counter = Counter { value: 0 };
    loop {
        println!("Inc or Dec?");
        let mut result = String::new();
        io::stdin().read_line(&mut result).expect("Error");
        match result.to_lowercase().trim() {
            "inc" => counter.update(Message::Increment),
            "dec" => counter.update(Message::Decrement),
            "exit" => break,
            _ => continue,
        };
        println!("Counter {}", counter.value);
    }
}
