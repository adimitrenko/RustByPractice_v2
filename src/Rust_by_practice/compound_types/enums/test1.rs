
enum Number {
    Zero = 0,
    One = 1,
    Two = 2,
}

#[test]
pub fn test39() {

    assert_eq!(Number::One as i32, 1);
    assert_eq!(Number::One as i32, 1);

    println!("Успіх!");
}
