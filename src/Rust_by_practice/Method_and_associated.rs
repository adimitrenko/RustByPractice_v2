struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[test]
pub fn test1() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}



#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self) {
        println!("the current state is {}", self.color);
    }
}
#[test]
pub fn test2() {
    let light = TrafficLight {
        color: "red".to_owned(),
    };

    light.show_state();

    println!("{:?}", light);
}




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



