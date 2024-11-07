use std::ops;

#[derive(PartialEq, Debug)]
struct FooBar;

#[derive(PartialEq, Debug)]
struct BarFoo;

struct Foo;
struct Bar;

impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

impl ops::Sub<Foo> for Bar {
    type Output = BarFoo;

    fn sub(self, _rhs: Foo) -> BarFoo {
        BarFoo
    }
}
#[test]
pub fn main() {
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Bar - Foo, BarFoo);

    println!("Success!");
}
