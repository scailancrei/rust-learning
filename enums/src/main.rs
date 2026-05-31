#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        dbg!(&self);
    }
}

fn main() {
    let write = Message::Write(String::from("hello world"));
    let move_ = Message::Move { x: 100, y: 20 };

    write.call();
    move_.call();
}
