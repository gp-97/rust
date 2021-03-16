fn square(n: i32) {
    println!("Square of {} = {}", n, i32::pow(n, 2));
}

fn cube(mut m: i32) {
    m += 4;
    println!("Cube of {} = {}", m, i32::pow(m, 3));
}

fn main() {
    let n = 5;
    square(n);
    let m = 6;
    cube(m);
}