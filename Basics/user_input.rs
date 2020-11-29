use std::io;

fn main() {
	let mut inp: String = String::new();
	io::stdin().read_line(&mut inp).expect("Invalid input");
	inp = inp[0..inp.len()-1].to_string();
	println!("Input: {:?}", inp);
}