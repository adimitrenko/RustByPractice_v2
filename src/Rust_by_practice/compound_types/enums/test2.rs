
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[test]
pub fn test40() {
    let _msg1 = Message::Move { x: 1, y: 2 }; // Ініціалізація з x = 1, y = 2
    let _msg2 = Message::Write(String::from("hello, world!")); // Ініціалізація з "hello, world!"

    println!("Успіх!");
}