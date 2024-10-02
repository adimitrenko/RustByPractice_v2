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
    let byte_escape = "I'm writing Ru\x73t!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("Unicode character {} (U+211D) is called {}", unicode_codepoint, character_name);

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

#[test]
pub fn test10() {
    let raw_str = "Escapes don't work here: ? ℝ";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    let long_delimiter = r"Hello";
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}

#[test]
pub fn test11() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // Take the first byte as a &str
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // Slice "中", which is 3 bytes in UTF-8
    assert_eq!(h1, "中");

    println!("Success!");
}

#[test]
pub fn test12() {
    for c in "你好，世界".chars() {
        println!("{}", c);
    }
}
