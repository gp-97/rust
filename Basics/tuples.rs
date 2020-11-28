/*
	-> Tuples are collections of values of different types.
	-> They can hold any number of variables.
	-> Best used when returning multiple values
*/
fn main() {
	let a = (1i32, 4u8, 16f32);
	println!("{:?}", a);

	let b = (5u32, );
	println!("{:?}", b);

	let c = (1, 2, 3, 4, 12, 13);
	println!("{:?}", c.0);

	let d = (a, (b, c));
	println!("{:?}", d);
}