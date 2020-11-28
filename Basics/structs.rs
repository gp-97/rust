use std::mem;

#[derive(Debug)]
struct Employee {
	name: String,
	age: u8,
	salary: f64,
}

// Unit struct
struct Simp;

// tuple struct
struct Point(f64, f64);

fn main() {
	let emp1 = Employee {
		name: String::from("John Doe"),
		age: 35,
		salary: 15000.52
	};

	println!("{:#?}", emp1);
	println!("emp1 = {:?} bytes", mem::size_of_val(&emp1));

	let point_a = Point(1.2, 3.4);
	println!("Point A\nx = {}, y = {}", point_a.0, point_a.1);

	// Destructuring a tuple struct

	let Point(x, y) = point_a;
	println!("x = {}, y = {:?}", x,y);

	let Simp = 

}