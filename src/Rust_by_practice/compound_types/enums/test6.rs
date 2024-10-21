enum List {
    // Cons: Кортежна структура, що обгортає елемент та вказівник на наступний вузол
    Cons(u32, Box<List>),
    // Nil: Вузол, що позначає кінець зв'язаного списку
    Nil,
}


impl List {
    // Створює порожній список
    fn new() -> List {
        List::Nil
    }

    // Споживає список і повертає той же список з новим елементом на початку
    fn prepend(self, elem: u32) -> List {
        List::Cons(elem, Box::new(self))
    }

    // Повертає довжину списку
    fn len(&self) -> u32 {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }

    // Повертає представлення списку як (розподілену в купі) строку
    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            List::Nil => {
                "Nil".to_string() // Замінено `format!` на `to_string`
            }
        }
    }
}

#[test]
pub fn test44() {

    let mut list = crate::rust_by_practice::compound_types::enums::test6::List::new();


    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);


    println!("Зв'язаний список має довжину: {}", list.len());
    println!("{}", list.stringify());
}