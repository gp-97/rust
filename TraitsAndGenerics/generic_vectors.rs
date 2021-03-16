use std::fmt::Display;

fn print_vec<T: Display>(vec: &Vec<T>) {
    for (idx, val) in vec.iter().enumerate() {
        println!("vec[{}]: {}", idx, val);
    }
}

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let v2: Vec<&str> = vec!["Once", "upon", "a", "time ..."];
    print_vec(&v1);
    print_vec(&v2);
}
