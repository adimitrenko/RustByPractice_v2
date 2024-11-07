struct Person {
    name: String,
    age: u8,
}

#[test]
pub fn test34() {
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };

    p.age = 30;

    p.name = String::from("sunfei");

    println!("Success!");
}