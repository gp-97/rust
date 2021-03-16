fn main() {
    let a: i32 = 10;
    let b: i32 = 12;

    if a > b {
        println!("a > b");
    } else if a == b {
        println!("a = b");
    } else {
        println!("a < b");
    }

    // Shorthand if..else
    let res = if a>b {"a > b"} else if a==b {"a == b"} else {"a < b"};
    
    println!("{}", res);
}