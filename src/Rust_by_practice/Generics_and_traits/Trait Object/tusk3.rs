trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn draw_with_box(x: Box<dyn Draw>) {
    println!("{}", x.draw());
}

fn draw_with_ref(x: &dyn Draw) {
    println!("{}", x.draw());
}
#[test]
pub fn main() {
    let x = 1.1f64;
    let y = 8u8;

    draw_with_box(Box::new(x));
    draw_with_ref(&y);

    println!("Success!");
}
