

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


// PATTERNS


#[test]
pub fn test10() {
    let n = 5; // тут замість зчитування з консолі можна передати тестове значення
    match_number(n);
}

fn match_number(n: i32) {
    match n {
        1 => println!("One!"),
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

#[test]
pub fn test11() {

    let p = Point { x: 3, y: 10 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),

        Point { x: 0..=5, y: y @ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Message2 {
    Hello { id: i32 },
}

#[test]
pub fn test12() {
    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello { id: id @ 3..=7 } => println!("Found an id in range [3, 7]: {}", id),
        Message2::Hello { id: id @ 10..=12 } => {
            println!("Found an id in another range [10, 12]: {}", id)
        }
        Message2::Hello { id } => println!("Found some other id: {}", id),
    }
}

#[test]
pub fn test13() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}

#[test]
pub fn test14() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}

#[test]
pub fn test15() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world!"),
    }

    println!("{}", v);
}
