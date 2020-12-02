fn calculate_length(s: &String) -> usize { //notice & in argument
	s.len()
}

fn append_string(s: &mut String) {
	s.push_str(". I enjoy rust.")
}

fn main() {
	let s1 = String::from("Hello Rust"); // s1 has ownership

	// let len = calculate_length(s1); // s1 moves ownership via function call

	/*
		s1 retains ownership and 'calculate_length()' only BORROWS reference to s1

	*/
	let len = calculate_length(&s1);

	println!("{:#?} length = {:?}", s1, len);

	let mut s2 = String::from("Hello Rust");

	/*
		Now as variables are mutable and immutable, so are references. 
		Thus if we want to make any changes to s1 when borrow occurs, we 
		need to pass a 'mut' reference of s2.
	*/
	append_string(&mut s2);
	println!("Modified s2  = {:?}", s2);

	/*
		NOTE:
		- RUST ONLY ALLOWS ONE MUTABLE REFERENCE AND THATS IT. THIS HELPS 
		  IN PREVENTING DATA RACES AT COMPILE TIME CHECKING.

		- WE CAN CREATE MULTIPLE IMUUTABLE REFERENCES THOUGH.

		- IF A VARIABLE ALREADY HAS A IMMUTABLE REFERENCE, THEN IT'S 
		  MUTABLE REFERENCE CANNOT BE CREATED.

		We can use curly brackets to create a new scope,
		allowing for multiple mutable references, just not simultaneous ones.
	*/


}
