struct Rectangle<T> {
    width: T,
    length: T,
}

fn main() {
    let r1: Rectangle<i32> = Rectangle {
        width: 2,
        length: 4,
    };
    let r2: Rectangle<f64> = Rectangle {
        width: 2.134134,
        length: 4314134.13412341,
    };
    println!("Width: {}, Height: {}", r1.width, r1.length);
    println!("Width: {}, Height: {}", r2.width, r2.length);
}
