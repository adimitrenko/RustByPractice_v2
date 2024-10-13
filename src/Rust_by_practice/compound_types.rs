// String
#[test]
pub fn test1() {
    let _s: &str = "hello, world";
    println!("Success!");
}

#[test]
pub fn test2() {
    let s: Box<str> = "hello, world".into();
    greetings(&s);
}


fn greetings(s: &str) {
    println!("{}", s);
}

#[test]
pub fn test3() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');
    assert_eq!(s, "hello, world!");
    println!("Success!");
}

#[test]
pub fn test4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";
    println!("{}", s);
}

#[test]
pub fn test5() {
    let s = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");
    assert_eq!(s1, "I like cats");
    println!("Success!");
}

#[test]
pub fn test6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}

#[test]
pub fn test7() {
    let s = "hello, world";
    greetings(&s.to_string());
}

#[test]
pub fn test8() {
    let s = "hello, world".to_string();
    let _s1: &str = &s;
    println!("Success!");
}

#[test]
pub fn test9() {
    let byte_escape = "Я пишу Ua\x73t!";
    println!("Що ти робиш\x3F (\\x3F тут ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("Unicode символ {} (U+211D) викликає {}", unicode_codepoint, character_name);

    let long_string = "Рядкові літерали
                        може охоплювати кілька рядків.
                        Розрив рядка та відступ тут \
                         також можна втекти";
    println!("{}", long_string);
}

#[test]
pub fn test10() {
    let raw_str = "Втечі тут не працюють: ? ℝ";
    assert_eq!(raw_str, "Втечі тут не працюють: ? ℝ");

    let quotes = r#"І тоді я сказав: «Немає порятунку!""#;
    println!("{}", quotes);

    let delimiter = r###"Рядок із "#". І навіть "##!"###;
    println!("{}", delimiter);

    let long_delimiter = r"Hello";
    assert_eq!(long_delimiter, "Hello");

    println!("Success!");
}

#[test]
pub fn test11() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1];
    assert_eq!(h, "h");

    let h1 = &s1[3..6];
    assert_eq!(h1, "中");

    println!("Success!");
}

#[test]
pub fn test12() {
    for c in "你好，世界".chars() {
        println!("{}", c);
    }
}

// Array

#[test]
pub fn test13() {

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    assert_eq!(arr.len() , 5);

    println!("Success!");
}

#[test]
pub fn test14() {

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    assert_eq!(arr.len() , 5);

    println!("Success!");
}

#[test]
pub fn test15() {

    let list: [i32; 100] = [1; 100];

    assert_eq!(list[0] , 1);
    assert_eq!(list.len() , 100);

    println!("Success!");
}

#[test]
pub fn test16() {

    let _arr = [1, 2, 3];

    println!("Success!");
}

#[test]
pub fn test17() {
    let arr = ['a', 'b', 'c'];

    let ele = arr[0];

    assert_eq!(ele , 'a');

    println!("Success!");
}

#[test]
pub fn test18() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    let _name1 = names.get(1).unwrap();

    println!("Success!");
}

// Slice

#[test]
pub fn test19() {
    let arr = [1, 2, 3];
    let _s1: &[i32] = &arr[0..2];

    let _s2: &str = "hello, world";

    println!("Success!");
}

#[test]
pub fn test20() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    assert_eq!(size_of_val(&slice), 16);

    println!("Success!");
}

#[test]
pub fn test21() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}

#[test]
pub fn test22() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}

#[test]
pub fn test23() {
    let s = "你好，世界";
    let slice = &s[0..3];

    assert_eq!(slice, "你");

    println!("Success!");
}

#[test]
pub fn test24() {
    let mut s = String::from("hello world");

    let letter = first_letter(&s);

    s.clear();

    println!("the first letter is: {}", letter);
}

fn first_letter(s: &str) -> String {
    s[..1].to_string()
}


// Tuple
#[test]
pub fn test25() {
    let _t0: (u8, i16) = (0, -1);
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    let _t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}

#[test]
pub fn test26() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    println!("Success!");
}

#[test]
pub fn test27() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}

#[test]
pub fn test28() {
    let tup = (1, 6.4, "hello");

    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}

#[test]
pub fn test29() {
    let (x, y, z);

    (y, z, x) = (1, 2, 3);

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}

#[test]
pub fn test30() {
    let (x, y) = sum_multiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

// Struct

#[allow(dead_code)]
struct Person {
    name: String,
    age: u8,
    hobby: String,
}

#[test]
pub fn test31() {
    let age = 30;
    let _p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding"),
    };

    println!("Success!");
}

struct Unit;
#[allow(dead_code)]
trait SomeTrait {

}

impl SomeTrait for Unit { }

#[test]
pub fn test32() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

fn do_something_with_unit(_u: Unit) { }

#[allow(dead_code)]
struct Color(i32, i32, i32);

#[allow(dead_code)]
struct Point(i32, i32, i32);

#[test]
pub fn test33() {
    let v = Color(0, 127, 255);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Color) {
    let x = p.0;
    let y = p.1;
    let z = p.2;

    assert_eq!(x, 0);
    assert_eq!(y, 127);
    assert_eq!(z, 255);
}



struct Person2 {
    name: String,
    age: u8,
}

#[test]
pub fn test34() {
    let age = 18;
    let mut p = Person2 {
        name: String::from("sunface"),
        age,
    };

    p.age = 30;

    p.name = String::from("sunfei");

    println!("Success!");
}

#[allow(dead_code)]
struct Person3 {
    name: String,
    age: u8,
}

#[test]
pub fn test35() {
    println!("Success!");
}


#[allow(dead_code)]
fn build_person(name: String, age: u8) -> Person3 {
    Person3 {
        age,
        name,
    }
}

#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[test]
pub fn test36() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let _u2 = set_email(u1);

    println!("Success!");
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[test]
pub fn test37() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("{:?}", rect1);
}

#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

#[test]
pub fn test38() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name = &f.name;

    println!("{}, {}", _name, f.data);
}

// Enum


#[allow(dead_code)]
enum Number {
    Zero,
    One,
    Two,
}

#[allow(dead_code)]
enum Number1 {
    Zero = 0,
    One = 1,
    Two = 2,
}

#[allow(dead_code)]
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}


#[test]
pub fn test39() {
    // Варіант енуму можна перетворити в ціле число через `as`
    assert_eq!(Number1::One as i32, 1);
    assert_eq!(Number2::One as i32, 1);

    println!("Успіх!");
}


#[allow(dead_code)]
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

#[allow(dead_code)]
enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[test]
pub fn test41() {
    let msg = Message2::Move { x: 1, y: 2 };

    if let Message2::Move { x: a, y: b } = msg {
        assert_eq!(a, 1);
        assert_eq!(b, 2);
    } else {
        panic!("Цього ніколи не має відбутись!");
    }

    println!("Успіх!");
}



enum Message3 {
    Quit,
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

#[test]
pub fn test42() {
    let msgs: [Message3; 3] = [
        Message3::Quit,
        Message3::Move { x: 1, y: 3 },
        Message3::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }

    println!("Успіх!");
}

fn show_message(msg: Message3) {
    match msg {
        Message3::Quit => println!("Quit"),
        Message3::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message3::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}

#[test]
pub fn test43() {
    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);
        println!("Успіх!");
    } else {
        panic!("Цього ніколи не має відбутись!");
    }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

enum List {
    // Cons: Кортежна структура, що обгортає елемент та вказівник на наступний вузол
    Cons(u32, Box<List>),
    // Nil: Вузол, що позначає кінець зв'язаного списку
    Nil,
}


impl List {
    // Створює порожній список
    fn new() -> List {
        List::Nil
    }

    // Споживає список і повертає той же список з новим елементом на початку
    fn prepend(self, elem: u32) -> List {
        List::Cons(elem, Box::new(self))
    }

    // Повертає довжину списку
    fn len(&self) -> u32 {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }

    // Повертає представлення списку як (розподілену в купі) строку
    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            List::Nil => {
                "Nil".to_string() // Замінено `format!` на `to_string`
            }
        }
    }
}

#[test]
pub fn test44() {

    let mut list = List::new();


    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);


    println!("Зв'язаний список має довжину: {}", list.len());
    println!("{}", list.stringify());
}
