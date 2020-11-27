fn add(a: i32, b: i32) -> i32 {
	a + b
}
fn sub(a: i32, b: i32) -> i32 {
	a - b
}
fn divide(a: i32, b: i32) -> i32 {
	a / b
}
fn product(a: i32, b: i32) -> i32 {
	a * b
}
fn main() {
	let a = 10;
	let b = 5;
	println!("Addition = {}", add(a, b));
	println!("Subtraction = {}", sub(a, b));
	println!("Multiplication = {}", product(a, b));
	println!("Division = {}", divide(a, b));

}