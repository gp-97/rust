fn main() {
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut vec2 = Vec::new();
    for i in &vec {
        vec2.push(*i);
    }
    println!("{:?}", vec2);
    println!("{:?}", vec);
    vec2[2] = 100;
    println!("{:?}", vec2);
    println!("{:?}", vec);
}