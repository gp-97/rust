fn main() {
	let a: i32 = 5;
	let mut b: i32 = 10;
	println!("b = {}", b);
	b = b * 2;
	// a = a * 2;
	//Above line leads to compile time error. Uncomment above line to check.
	println!("a = {}", a);
	println!("b = {}", b);
}