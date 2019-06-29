#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move{ x: x1, y: y1 } => println!("Move: {}, {}", x1, y1),
            Message::Write(msg) => println!("The message says: {}'", msg),
            Message::ChangeColor(r, g, b) =>
                println!("ChangeColor: {}, {}, {}", r, g, b),
        }
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.print();
}

