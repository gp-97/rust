fn check(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err(String::from("[ERROR]: Division by zero"))
    } else {
        Ok(x / y)
    }
}

fn main() {
    let x = 100;
    let y = 0;
    match check(x, y) {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e),
    };
}
