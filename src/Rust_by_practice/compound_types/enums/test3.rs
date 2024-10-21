
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[test]
pub fn test41() {
    let msg = Message::Move { x: 1, y: 2 };

    if let Message::Move { x: a, y: b } = msg {
        assert_eq!(a, 1);
        assert_eq!(b, 2);
    } else {
        panic!("Цього ніколи не має відбутись!");
    }

    println!("Успіх!");
}
