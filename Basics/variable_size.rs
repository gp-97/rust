use std::mem;

fn main() {
	let a = 10;
	println!("size of {} = {:?} bytes", a, mem::size_of_val(&a));
	let b: u128 = 15;
	println!("size of {:?} = {:?} bytes", b, mem::size_of_val(&b));

}