// Take string user input and change case of each character

use std::io;

fn get_input() -> String {
	let mut inp = String::new();
	io::stdin().read_line(&mut inp).expect("Failed!!!");
	inp.trim().to_string()
}
fn main() {
	let inp: String = get_input();
	let mut out: String = String::new();
	for i in inp.bytes() {
		if i >= 65 && i <=90 {
			out.push((i + 32) as char);
		}
		else if i >= 97 && i <= 122{
			out.push((i - 32) as char);
		}
		else if i == 32 {
			out.push(32 as char);
		}
	}
	println!("{}", out);
}