#[test]
fn count_apple_and_orange(s: i32, t: i32, a: i32, b: i32, apples: &[i32], orange: &[i32])  {
    let mut apple_count = 0;

    for da in apples {
        let c = a+da;
        if c>= s && c <=t {
            apple_count += 1;
        }
    }

    let mut orange_count = 0;
    for db in apples {
        let c = a+db;
        if c>= s && c <=t {
            orange_count += 1;
        }
    }

    println!("{}\n{}", apple_count, orange_count);

}