fn main() {
	/*
		Checking scope
	*/
	// If we try to print s outside the curly brackets, we get an error
	{
		let s = 2;
		println!("{:?}", s);
	}
	// println!("{:?}", s);
}