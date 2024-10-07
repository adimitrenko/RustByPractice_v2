#[test]
pub fn run () {
    const SIZE: u8 = 8;

    for y in 0..SIZE {
        for x in 0..SIZE {
            let s = match (x + y) % 2 {
                0 => "**",
                _ => " ",
            };
            println!("{s}");
        }
        println!();
    }
}