use std::ops::Sub;

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

// Implementing Sub with default generic type parameters.
impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[test]
pub fn main() {
    assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
               Point { x: 1, y: 3 });

    println!("Success!");
}
