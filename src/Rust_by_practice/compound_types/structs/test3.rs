
struct Color(i32, i32, i32);


struct Point(i32, i32, i32);

#[test]
pub fn test33() {
    let v = Color(0, 127, 255);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Color) {
    let x = p.0;
    let y = p.1;
    let z = p.2;

    assert_eq!(x, 0);
    assert_eq!(y, 127);
    assert_eq!(z, 255);
}