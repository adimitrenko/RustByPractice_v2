trait MyTrait {
    fn f(&self) -> Box<dyn MyTrait>;
}

impl MyTrait for u32 {
    fn f(&self) -> Box<dyn MyTrait> { Box::new(42u32) }
}

impl MyTrait for String {
    fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) }
}

fn my_function(x: Box<dyn MyTrait>) {
    let _ = x.f();
}
#[test]
pub fn main() {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));

    println!("Success!");
}
