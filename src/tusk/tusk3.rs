#[test]
pub fn run() {
    const SIZE: u8 = 5;

    for y in 0..SIZE * 2 - 1 {
        let row_size = if y < SIZE {
            y + 1
        } else {
            SIZE * 2 - y - 1
        };

        for x in 0..SIZE * 2 - 1 {
            let s = if (SIZE - row_size..SIZE + row_size).contains(&x) {
                "*"
            } else {
                " "
            };
            print!("{s}");
        }
        println!();
    }
}
