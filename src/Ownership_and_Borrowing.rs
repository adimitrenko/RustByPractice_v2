

pub fn test1() {

    let x = String::from("Hello world");
    let y = &x;
    println!("{}, {}", x, y);
}

pub fn test2() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

pub fn test3() {
    let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from("Hello world");

    let _s = s.clone().into_bytes();
    s
}

pub fn test4() {
    let s = String::from("Hello World");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &String) {
    println!("{}", s)
}

pub fn test5() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}



pub fn test6() {
    let mut s = String::from("Hello ");
    s.push_str("World!");

    println!("Success!");
}

pub fn test7() {
    let x = Box::new(5);
    let mut y = Box::new(5); // create a mutable Box

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}


pub fn test8() {
    let t = (String::from("hello"), String::from("world"));

    let s = t.0.clone();

    println!("{:?}, {:?}", s, t);
}

pub fn test9() {
    let t = (String::from("hello"), String::from("world"));

    let (s1, s2) = (&t.0, &t.1);

    println!("{:?}, {:?}, {:?}", s1, s2, t);
}


pub fn test10() {
    let x = 5;
    let p = &x;

    println!("the memory address of x is {:p}", p);
}

pub fn test11() {
    let x = 5;
    let y = &x;

    assert_eq!(5, *y);

    println!("Success!");
}
