
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