struct Rectangle {
    width: f32,
    height: f32,
}

fn main() {
    // Immutable struct declaration
    let s1 = Rectangle {
        width: 23.0,
        height: 30.2,
    };
    println!("Area: {}", s1.width * s1.height);

    // Mutable struct declration 
    let mut s2 = Rectangle {
        width: 30.0,
        height: 15.5
    };
    s2.height *= 2 as f32;
    println!("Area: {}", s2.width * s2.height);
}
