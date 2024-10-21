struct Rectangles {
    width: u32,
    height: u32,
}


impl Rectangles {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangles {
    fn can_hold(&self, other: &Rectangles) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[test]
pub fn test5() {
    println!("Success!");
}