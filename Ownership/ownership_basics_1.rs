#[allow(unused)]
fn main() {
	/*
		Checking scope
	*/
	// If we try to print s outside the curly brackets, we get an error.
	{
		let s = 2;
		println!("{:?}", s);
	}
	// println!("{:?}", s);

	/*
		Rust frees memory after a variable goes out of scope.
		The concept of ownership is intended for cases when a
		variable stores data on the heap memory. 

		Consider the case of &str and String in rust. 
		&str stores string literals on memory stack, whereas 
		String stores data in heap. Now in cases we have such a condition:
	*/
	{
		let s1 = String::from("Stored on heap");
		let s2 = s1;
	}
	/*
		The code above allocates s1 the string and creates a reference of 
		it and assigns it to s2. Now if we go outside the scope, then both
		s1 and s2 will try to free up the same memory. This may lead to data 
		corruption.

		To solve this dillema, rust 'moves' the ownership of data in s1 to s2
		and then it is s2 (or the last variable holdingthe reference) that frees up 
		the heap memory and return it back to the system.

		Now we have transferred the ownership of "Hello World" from s1 to s2. 
		Thus s1 no longer has access to data "Hello World". 
		NOTE:- It all happens because `copy` trait wasn't implemented for s1.
	*/

	/*
		In order to perform a deep copy, use 'clone'
	*/
	{
		let s1 = String::from("Hello World");
		let s2 = s1.clone();
	}
	/*
		
	*/

}