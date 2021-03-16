fn swap(n1: &mut i32, n2: &mut i32) {
    let temp = *n1;
    *n1 = *n2;
    *n2 = temp;
}

fn main() {
    let mut n1 = 5;
    let mut n2 = 10;
    println!("Before swap, n1: {}, n2: {}", n1, n2);
    swap(&mut n1, &mut n2);
    println!("After swap, n1: {}, n2: {}", n1, n2);
}