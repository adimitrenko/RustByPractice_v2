//match, matches...
#[allow(dead_code)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[test]
pub fn test1() {
    let dire = Direction::East;
    match dire {
        Direction::East => println!("East"),
        _ => (),
    };
}

#[test]
pub fn test2() {
    let boolean = true;

    let binary = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

    println!("Success!");
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[test]
pub fn test3() {
    let msg = Message::Write(String::from("Hello"));
    if let Message::Write(content) = msg {
        assert_eq!(content, "Hello");
    }

    let msgs = vec![
        Message::Write(String::from("Hello")),
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }

    println!("Success!");
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => {
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        _ => println!("no data in these variants"),
    }
}


#[test]
pub fn test4() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'));
    }

    println!("Success!");
}


enum MyEnum {
    Foo,
    Bar,
}

#[test]
pub fn test5() {
    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}

#[test]
pub fn test6() {
    let o = Some(7);

    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
    }
}


enum Foo {
    Bar(u8),
}

#[test]
pub fn test7() {
    let a = Foo::Bar(1);

    let Foo::Bar(i) = a;

    println!("foobar holds the value: {}", i);
    println!("Success!");
}


#[allow(dead_code)]
enum Foo2 {
    Bar,
    Baz,
    Qux(u32),
}

#[test]
pub fn test8() {
    let a = Foo2::Qux(10);

    match a {
        Foo2::Bar => println!("match foo::bar"),
        Foo2::Baz => println!("match foo::baz"),
        _ => println!("match others"),
    }
}

#[test]
pub fn test9() {
    let age = Some(30);
    if let Some(age_value) = age {
        assert_eq!(age_value, 30);
    }

    match age {
        Some(age) => println!("age is a new variable, it's value is {}", age),
        _ => (),
    }
}

