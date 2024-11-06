struct Array<T, const N: usize> {
    data: [T; N],
}

enum ArrayEnum {
    IntArray(Array<i32, 3>),
    FloatArray(Array<f64, 3>),
}

#[test]
pub fn main() {
    let arrays = [
        ArrayEnum::IntArray(Array { data: [1, 2, 3] }),
        ArrayEnum::FloatArray(Array { data: [1.0, 2.0, 3.0] }),
    ];

    println!("Успіх!");
}