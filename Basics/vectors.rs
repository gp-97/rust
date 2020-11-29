use std::io;

fn get_input() -> String {
	let mut inp = String::new();
	io::stdin().read_line(&mut inp).expect("Failed");
	inp.trim().to_string()
}

fn main() {

	let mut chars: Vec<char> = get_input().chars().collect();
	println!("{:?}", chars);
	chars.push('Y');
	println!("{:?}", chars);
}