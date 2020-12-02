#[derive(Debug)]
struct Rectangle {
	length: f64,
	width: f64,
}

impl Rectangle {
	fn area(&self) -> f64 {
		self.length * self.width
	}
	fn perimeter(&self) -> f64 {
		2.0 * (self.length + self.width)
	}
	fn diagonal(&self) -> f64 {
		((self.length * self.length) + (self.width * self.width)).sqrt()
	}
}

fn main() {
	let rect = Rectangle {
		length: 10.24,
		width: 20.48
	};
	println!("{:#?}", rect);
	println!("area = {:?}", rect.area());
	println!("perimeter = {:?}", rect.perimeter());
	println!("diagonal length = {:?}", rect.diagonal());
}