pub fn test04() {
    let x = define_x();
    println!("{}, світ", x);
}

fn define_x() -> &'static str {
    "привіт"
}
