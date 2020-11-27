/*
	String Objects unlike String literals are
	not a part of the standard library. They 
	are defined in the public structure in standard 
	library as `pub struct String`.
*/
fn main() {
	// String literal (Used when value known at compile time)
	let first_name: &str = "John";

	// String object (Used when value is provided at run time)
	let last_name = String::from("Doe");
	let fullname = format!("{} {}", first_name.to_string(), last_name);
	println!("Full name: {}", fullname);

	// To create an empty string object and assign it a value(notice 'mut')
	let mut empty_string_object = String::new();
	empty_string_object.push_str("Once upon a time there lived no one");
	println!("{}\nLength = {}", empty_string_object, empty_string_object.len());

	let tokens: Vec<&str> = empty_string_object.split_whitespace().collect();
	for token in tokens {
		println!("{}", token.to_string());
	}

	let mut line = String::new();
	line.push_str("1,2,3,4,5,6,7,8,9,10");
	let numbers: Vec<&str> = line.split(",").collect();
	for number in numbers {
		println!("{}", number);
	}

}