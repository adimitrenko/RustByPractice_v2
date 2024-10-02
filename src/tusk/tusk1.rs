#[test]
pub fn run() {

    const W: u32 = 25;
    const H: u32 = 10;

    let k: f32  = W as f32 / H as f32;

    for y in 0..H {
        for x in 0..W {

            let is_horizontal = y == 0 || y == H - 1;
            let is_vertical = x == 0 || x == W - 1;
            let id_diagonal: bool = x ==(y as f32 *k) as u32;
            let id_co_diagonal: bool = x == W - 1 - (y as f32 *k) as u32;

            let c = if is_horizontal || is_vertical || id_diagonal || id_co_diagonal {
                "*"
            } else {
                " "
            };
            print!("{}", c );
        }
        println!();
    }
}
