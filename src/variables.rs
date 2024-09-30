#[test]
pub fn test01() {
    let x: i32 = 5;
    let _y: i32;

    assert_eq!(x, 5);
    println!("Успіх!");
}

#[test]
pub fn test02() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Успіх!");
}

#[test]
pub fn test03() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("Значення x: {} і значення y: {}", x, y);
    }
    println!("Значення x: {}", x);
}

#[test]
pub fn test04() {
    let x = define_x();
    println!("{}, світ", x);
}

fn define_x() -> &'static str {
    "привіт"
}

#[test]
pub fn test05() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Виводить "42".
}

#[test]
pub fn test06() {
    let mut x: i32 = 1;
    x = 7;

    let _x = x;
    let _y = 4;

    let _y = "Я також можу бути текстом!";

    println!("Успіх!");
}

#[test]
pub fn test07() {
    let x = 1;
    println!("{}", x); // Використовуємо змінну для виведення її значення
}

#[test]
pub fn test08() {
    let (mut x, y) = (1, 2); // Додаємо mut для змінної x
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

#[test]
pub fn test09() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // Заповнюємо пропуск
    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}


