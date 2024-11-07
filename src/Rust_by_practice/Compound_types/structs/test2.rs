struct Unit;

trait SomeTrait {

}

impl Unit {

}

#[test]
pub fn test32() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

fn do_something_with_unit(_u: Unit) { }