#[allow(unused_variables, unused_mut)]

fn main() {
    let t1: (&str, char, f32) = ("Rust", 'A', 100.00);
    let t2 = (12, 23.3434, "Hello");

    println!("{:?}", t1);
    println!("{:?}", t2);

    // Acessing values
    let (a, b, c) = t1;
    let val_t2 = t2.2;

    println!("{}, {}, {}", a, b, c);
    println!("{:?}", val_t2);

}