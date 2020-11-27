fn main() {

	// for loop
	for i in 1..10 {
		print!("{}\t", i);
	}
	println!();

	// while loop
	let mut i = 1;
	while i <= 10 {
		println!("{}", i);
		i += 1;
	}
}