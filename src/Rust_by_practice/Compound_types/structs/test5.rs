
struct Person {
    name: String,
    age: u8,
}

#[test]
pub fn test35() {
    println!("Success!");
}



fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name,
    }
}