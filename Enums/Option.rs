use std::mem;

fn check(x: Option<i32>) -> Option<i32> {
	match x {
		Some(a) => {
			println!("{:?}", a);
			Some(a)
		},
		None => None,
	}
}

fn main() {
	let x: Option<i32> = Some(10);
	let ret = check(x);
	println!("{:?} = {:?} bytes", ret, mem::size_of_val(&ret));
}