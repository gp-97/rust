fn main() {
	let day: &str = "Tuesday";

	let res = match day {
		"monday" => {
			println!("Found it"); "Its monday"
		},
		_ => "LoL gotem!!!"
	};
	println!("The day is......{}", res);
}