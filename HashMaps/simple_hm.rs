use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<String, i32> = HashMap::new();
    hm.insert(String::from("Bruh"), 10);
    println!("{:?}", hm);

    let mut hm = HashMap::new();
    hm.insert(String::from("Bruh"), 10);
    println!("{:?}", hm);

    let mut hm = HashMap::<String, u8>::new();
    hm.insert(String::from("Bruh"), 10);
    println!("{:?}", hm);

    let mut hm: HashMap<String, i32> = HashMap::<String, i32>::new();
    hm.insert(String::from("Bruh"), 10);
    println!("{:?}", hm);

    let mut scores = HashMap::new();
    scores.insert('a', 1);
    scores.insert('b', 2);
    scores.insert('c', 3);
    scores.insert('d', 4);

    println!("{:#?}", scores);
    match scores.get(&'a') {
        Some(val) => println!("{}", val),
        None => println!("No such key"),
    };
    match scores.get_key_value(&'b') {
        Some(tup) => println!("{:?}", tup),
        None => println!("No such key value pair present"),
    }

    for (team, score) in &scores {
        println!("{}: {}", team, score);
    }
    println!("{:#?}", scores);
}
