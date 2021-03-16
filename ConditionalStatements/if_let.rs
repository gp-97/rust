fn main() {
    let a = (1, 2, 3);
    if let(1, b, c) = a{
        println!("b: {}\nc: {}", b, c);
    } else {
        println!("Pattern unmatched");
    }
}