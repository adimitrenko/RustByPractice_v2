#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[test]
pub fn test37() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect);

    println!("{:?}", rect);
}