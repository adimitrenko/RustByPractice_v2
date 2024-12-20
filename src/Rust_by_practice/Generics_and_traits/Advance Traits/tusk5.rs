use std::fmt;

struct Pretty(String);

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}
#[test]
pub fn main() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}
