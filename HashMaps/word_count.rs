use std::collections::HashMap;

fn main() {
    let str = "Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data. Here are some exercises you should now be equipped to solve:";
    let mut map = HashMap::<&str, i32>::new();
    for word in str.split_whitespace() {
        let val_ref = map.entry(word).or_insert(0);
        *val_ref += 1;
    }
    println!("{:#?}", map);
}
