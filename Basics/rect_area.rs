#[derive(Debug)]
struct Point {
	x: f64,
	y: f64,
}

#[derive(Debug)]
struct Rectangle {
	top_left: Point,
	bottom_right: Point,
}

fn create_rectangle(top_left: Point, bottom_right: Point) -> Rectangle {
	Rectangle {top_left: top_left, bottom_right: bottom_right}
}

fn main() {

	let top_left = Point {x: 1.2, y: 4.8};
	let bottom_right = Point {x: 16.32, y: 64.128};
	println!("Point top_left = {:?}", top_left);
	println!("Point bottom_right = {:?}", bottom_right);

	
	let rect = create_rectangle(top_left, bottom_right);
	println!("Rectangle: {:#?}", rect);

	// Calculating the rectangle's area
	let length: f64 = (rect.bottom_right.x - rect.top_left.x).abs();
	let width: f64 = (rect.bottom_right.y - rect.top_left.y).abs();
	let area: f64 = length * width;
	println!("Area = {:?}", area);
}