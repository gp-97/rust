use std::collections::HashMap;

fn main() {
    let key: Vec<&str> = vec!["A", "B", "C", "D", "E"];
    let val: Vec<i32> = vec![1, 2, 3, 4, 5];
    let map: HashMap<_, _> = key.into_iter().zip(val.into_iter()).collect();
    println!("{:#?}", map);
}
