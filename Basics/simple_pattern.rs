use std::io;

fn get_int_input() -> i64 {
	let mut inp = String::new();
	io::stdin().read_line(&mut inp).expect("Failed !!!");
	inp.trim().parse::<i64>().unwrap()
}

fn main() {
	let lim = get_int_input();
	for i in 1..lim+1 {
		for j in 1..i+1 {
			print!("{}\t", j);
		}
		println!();
	}
	println!();
	let mut space = lim;
	for _ in 1..lim+1 {
		for _ in 1..space {
			print!("\t");
		}
		for _ in space..lim+1 {
			print!("*\t");
		}
		space -= 1;
		println!();
	}
}