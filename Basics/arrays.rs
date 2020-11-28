/*
Arrays:-
	1. Contiguous memory space allocation.
	2. Length known at compile time.
	3. Signature:- [T; length]

Slices:-
	1. Length not known at compile time.
	2. Signature:- &[T]
		a. First word is a pointer to the data.
		b. Second word is length of slice.
*/

use std::mem;

fn analyze_slice(sl: &[f64]) {
	println!("\nslice = {:?}", sl);
	println!("slice length = {:?}", sl.len());
	println!("slice size = {:?} bytes", mem::size_of_val(&sl));
}

fn main() {
	let a: [f64; 10] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
	println!("arrays = {:?}", a);
	println!("array length = {:?}", a.len());
	println!("array size = {:?} bytes", mem::size_of_val(&a));
	println!("fifth element = {:?}", a[5]);

	analyze_slice(&a[3..5]);
}

