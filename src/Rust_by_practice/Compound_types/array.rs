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