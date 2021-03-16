fn main() {
    let num_array: Vec<u8> = vec![1, 2, 3, 4, 5];
    for j in 0..num_array.len() {
        println!("{:?}", num_array[j]);
    }
    match num_array.get(9) {
        Some(n) => println!("{}", n),
        None => println!("Wrong index accessed")
    }
}