// Struct

mod test1;
mod test2;
mod test3;
mod test4;
mod test5;
mod test6;
mod test7;
mod test8;

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

impl crate::rust_by_practice::compound_types::SomeTrait for crate::rust_by_practice::compound_types::Unit { }

#[test]
pub fn test32() {
    let u = crate::rust_by_practice::compound_types::Unit;
    crate::rust_by_practice::compound_types::do_something_with_unit(u);

    println!("Success!");
}

fn do_something_with_unit(_u: crate::rust_by_practice::compound_types::Unit) { }

#[allow(dead_code)]
struct Color(i32, i32, i32);

#[allow(dead_code)]
struct Point(i32, i32, i32);

#[test]
pub fn test33() {
    let v = crate::rust_by_practice::compound_types::Color(0, 127, 255);
    crate::rust_by_practice::compound_types::check_color(v);

    println!("Success!");
}

fn check_color(p: crate::rust_by_practice::compound_types::Color) {
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
    let mut p = crate::rust_by_practice::compound_types::Person2 {
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
fn build_person(name: String, age: u8) -> crate::rust_by_practice::compound_types::Person3 {
    crate::rust_by_practice::compound_types::Person3 {
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
    let u1 = crate::rust_by_practice::compound_types::User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let _u2 = crate::rust_by_practice::compound_types::set_email(u1);

    println!("Success!");
}

fn set_email(u: crate::rust_by_practice::compound_types::User) -> crate::rust_by_practice::compound_types::User {
    crate::rust_by_practice::compound_types::User {
        email: String::from("contact@im.dev"),
        ..u
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

#[test]
pub fn test37() {
    let scale = 2;
    let rect1 = crate::rust_by_practice::compound_types::Rectangle {
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
    let f = crate::rust_by_practice::compound_types::File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name = &f.name;

    println!("{}, {}", _name, f.data);
}