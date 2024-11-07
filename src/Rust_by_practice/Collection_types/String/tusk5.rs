#[test]
pub fn main() {
    let mut s = String::new();
    s.push_str("hello");

    let v = vec![104, 101, 108, 108, 111];

    let s1 = String::from_utf8(v).expect("Invalid UTF-8 sequence");

    assert_eq!(s, s1);

    println!("Success!");
}
