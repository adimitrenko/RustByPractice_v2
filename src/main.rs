mod variable01;
mod variable02;
mod variable03;
mod variable04;
mod variable05;
mod variable06;

fn main() {
    println!("Запуск test01:");
    variable01::test01();

    println!("\nЗапуск test02:");
    variable02::test02();

    println!("\nЗапуск test03:");
    variable03::test03();

    println!("\nЗапуск test04:");
    variable04::test04();

    println!("\nЗапуск test05:");
    variable05::test05();

    println!("\nЗапуск test06:");
    variable06::test06();

    println!("Всі тести пройдені успішно!");
}
