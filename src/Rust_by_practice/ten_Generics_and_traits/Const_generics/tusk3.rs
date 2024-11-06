#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn check_size<T>(val: T) {
    if core::mem::size_of::<T>() < 768 {
        println!("Розмір даних дозволений.");
    } else {
        panic!("Розмір даних перевищує ліміт!");
    }
}

#[test]
pub fn main() {
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 96]); // приблизно 8 байтів * 96 = 768 байтів
    check_size([(); 48].map(|_| "hello你好".to_string())); // оцінка
    check_size(['中'; 192]); // приблизно 4 байти * 192 = 768 байтів

    println!("Успіх!");
}
