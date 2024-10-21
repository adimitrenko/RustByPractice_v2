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
