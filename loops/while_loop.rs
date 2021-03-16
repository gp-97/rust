fn main() {
    let mut var = 1;
    while var < 10 {
        if var % 2 == 0 {
            println!("{}", var);
        }
        var += 1;
    }
}