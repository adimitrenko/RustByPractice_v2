pub fn test03() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("Значення x: {} і значення y: {}", x, y);
    }
    println!("Значення x: {}", x);
}
