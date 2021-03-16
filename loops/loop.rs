fn main() {
    let mut var = 0;
    loop {
        var += 1;
        println!("{}", var);
        if var % 15 == 0 {
            break;
        }
    }
}