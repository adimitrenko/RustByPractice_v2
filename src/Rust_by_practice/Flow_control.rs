#[test]
pub fn test1() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

#[test]
pub fn test2() {
    let n = 5;

    let big_n =
        if n < 10 && n > -10 {
            println!(", і це невелике число, збільшити в десять разів");
            10 * n
        } else {
            println!(", і це велике число, поділіть це число вдвічі");
            n / 2
        };

    println!("{} -> {}", n, big_n);
}

#[test]
#[should_panic(expected = "НІКОЛИ НЕ ДОПУСКАЙТЕ ЦЬОГО")]
pub fn test3() {
    for n in 1..=100 {
        if n == 100 {
            panic!("НІКОЛИ НЕ ДОПУСКАЙТЕ ЦЬОГО");
        }
    }

    println!("Success!");
}


#[test]
pub fn test4() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    for _name in &names {
        // Do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy, so there is no move here
    for _n in numbers {
        // Do something with n...
    }

    println!("{:?}", numbers);
}

#[test]
pub fn test5() {
    let a = [4, 3, 2, 1];

    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }
}

#[test]
pub fn test6() {

    let mut n = 1;

    while n < 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }

    println!("n reached {}, so loop is over", n);
}

#[test]
pub fn test7() {
    let mut n = 0;
    for _i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}

#[test]
pub fn test8() {
    let mut n = 0;
    for _i in 0..=100 {
        if n != 66 {
            n += 1;
            continue;
        }

        break;
    }

    assert_eq!(n, 66);

    println!("Success!");
}

#[test]
pub fn test9() {
    let mut count = 0u32;

    println!("Будемо рахувати до нескінченності!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Добре");

            break;
        }
    }

    assert_eq!(count, 5);

    println!("Success!");
}

#[test]
pub fn test10() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}

#[test]
pub fn test11() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }

        count += 5;

        loop {
            if count >= 30 {
                break 'outer;
            }

            continue 'outer;
        }
    }

    assert_eq!(count, 30);

    println!("Success!");
}
