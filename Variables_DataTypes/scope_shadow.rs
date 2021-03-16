fn main() {
    let outer = 10;
    {
        let mut outer = 15;
        println!("{}", outer);
        outer += 12;
        println!("{}", outer);
    }
    println!("{}", outer);
}
