mod variables;
mod basic_types;
mod tusk1;
mod ownership_and_borrowing;
mod compound_types;

use std::io::{self, Write};

fn main() {
    // Виводимо підказку для користувача
    println!("Введіть номер для запуску:");
    println!("1 - Запустити тести variables");
    println!("2 - Запустити тести basic types");
    println!("3 - Запустити tusk1");
    println!("4 - Запустити тести Ownership and Borrowing");
    println!("5 - Запустити тести Compound Types");

    // Зчитуємо введення користувача
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Очищуємо буфер, щоб вивести текст до введення
    io::stdin().read_line(&mut input).expect("Помилка читання вводу");

    // Обрізаємо зайві символи і перетворюємо на число
    let input = input.trim().parse::<u32>().unwrap_or(0);

    match input {
        1 => {
            println!("Запуск тестів variables:");
            run_variables_tests();
        }
        2 => {
            println!("Запуск тестів basic types:");
            run_basic_types_tests();
        }
        3 => {
            println!("Запуск tusk1:");
            tusk1::run(); // Припускаємо, що в tusk1.rs є функція run
        }
        4 => {
            println!("Запуск тестів Ownership and Borrowing:");
            run_ownership_and_borrowing_tests();
        }
        5 => {
            println!("Запуск тестів Compound Types:");
            run_compound_types_tests();
        }
        _ => {
            println!("Неправильний вибір. Будь ласка, введіть 1, 2, 3, 4 або 5.");
        }
    }

    println!("Всі тести пройдені успішно!");
}

fn run_variables_tests() {
    println!("Запуск variables test01:");
    variables::test01();

    println!("\nЗапуск variables test02:");
    variables::test02();

    println!("\nЗапуск variables test03:");
    variables::test03();

    println!("\nЗапуск variables test04:");
    variables::test04();

    println!("\nЗапуск variables test05:");
    variables::test05();

    println!("\nЗапуск variables test06:");
    variables::test06();

    println!("\nЗапуск variables test07:");
    variables::test07();

    println!("\nЗапуск variables test08:");
    variables::test08();

    println!("\nЗапуск variables test09:");
    variables::test09();
}

fn run_basic_types_tests() {
    println!("\nЗапуск basic types test01:");
    basic_types::test01();

    println!("\nЗапуск basic types test02:");
    basic_types::test02();

    println!("\nЗапуск basic types test03:");
    basic_types::test03();

    println!("\nЗапуск basic types test04:");
    basic_types::test04();

    println!("\nЗапуск basic types test05:");
    basic_types::test05();

    println!("\nЗапуск basic types test06:");
    basic_types::test06();

    println!("\nЗапуск basic types test07:");
    basic_types::test07();

    println!("\nЗапуск basic types test08:");
    basic_types::test08();

    println!("\nЗапуск basic types test09:");
    basic_types::test09();

    println!("\nЗапуск basic types test10:");
    basic_types::test10();

    println!("\nЗапуск basic types test11:");
    basic_types::test11();
}

fn run_ownership_and_borrowing_tests() {
    println!("Запуск Ownership and Borrowing test1:");
    ownership_and_borrowing::test1();

    println!("\nЗапуск Ownership and Borrowing test2:");
    ownership_and_borrowing::test2();

    println!("\nЗапуск Ownership and Borrowing test3:");
    ownership_and_borrowing::test3();

    println!("\nЗапуск Ownership and Borrowing test4:");
    ownership_and_borrowing::test4();

    println!("\nЗапуск Ownership and Borrowing test5:");
    ownership_and_borrowing::test5();

    println!("\nЗапуск Ownership and Borrowing test6:");
    ownership_and_borrowing::test6();

    println!("\nЗапуск Ownership and Borrowing test7:");
    ownership_and_borrowing::test7();

    println!("\nЗапуск Ownership and Borrowing test8:");
    ownership_and_borrowing::test8();

    println!("\nЗапуск Ownership and Borrowing test9:");
    ownership_and_borrowing::test9();

    println!("\nЗапуск Ownership and Borrowing test10:");
    ownership_and_borrowing::test10();

    println!("\nЗапуск Ownership and Borrowing test11:");
    ownership_and_borrowing::test11();
}

// Нова функція для запуску тестів Compound Types
fn run_compound_types_tests() {
    println!("\nЗапуск Compound Types test1:");
    compound_types::test1();

    println!("\nЗапуск Compound Types test2:");
    compound_types::test2();

    println!("\nЗапуск Compound Types test3:");
    compound_types::test3();

    println!("\nЗапуск Compound Types test4:");
    compound_types::test4();

    println!("\nЗапуск Compound Types test5:");
    compound_types::test5();

    println!("\nЗапуск Compound Types test6:");
    compound_types::test6();

    println!("\nЗапуск Compound Types test7:");
    compound_types::test7();

    println!("\nЗапуск Compound Types test8:");
    compound_types::test8();

    println!("\nЗапуск Compound Types test9:");
    compound_types::test9();

    println!("\nЗапуск Compound Types test10:");
    compound_types::test10();

    println!("\nЗапуск Compound Types test11:");
    compound_types::test11();

    println!("\nЗапуск Compound Types test12:");
    compound_types::test12();
}
