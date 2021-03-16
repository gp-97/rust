fn main() {
    let mut pd: u128 = 1;
    for i in 1..6 {
        pd *= i;
    }
    println!("Factorial(5): {}", pd);

    // With Enumeration
    pd = 1;
    for (count, variable) in (1..11).enumerate() {
        pd *= variable;
        println!("Factorial of {} at idx {}: {}", variable, count, pd);
    }
}