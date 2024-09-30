#[test]
fn main() {
    for y in 0..10 {
        for x in 0..10 {
            if (x + y) % 2 == 0 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
