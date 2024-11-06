#[test]
pub fn main() {
    assert_eq!(sum(1, 2), 3);
}

fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}
