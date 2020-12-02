/*
	MeshUnit is a Rust enum that can be used to
	define the type of mesh unit before rendering.

	While rendering the model, the mesh unit can be 
	either a point or a line or a triangle or a quadrilateral.
	It can't be	of multiple types at the same time for the same 
	mesh unit.

	Thus MeshUnit is now a custom datatype that can be 
	used anywhere in the code.
*/

#[derive(Debug)]
struct Point {
	id: usize,
	vertex: Vec<i64>,
	color: Vec<u8>,
}

#[derive(Debug)]
enum MeshUnit<'a> {
	Line(Vec<&'a Point>),
	Triangle(Vec<&'a Point>),
	Quadrilateral(Vec<&'a Point>),
}

fn main() {
	/*
		Enum variants can hold value types.
		For example, each MeshUnit variant can hold values like
		1. index
		2. co-ordinates
		3. color
	*/

	let p1: Point = Point {
		id: 1,
		vertex: vec![1, 0, 1],
		color: vec![255, 0, 0]
	};
	let p2: Point = Point {
		id: 2,
		vertex: vec![3, -1, 1],
		color: vec![0, 255, 0]
	};
	let p3: Point = Point {
		id: 3,
		vertex: vec![-2, 2, -1],
		color: vec![0, 0, 255]
	};
	let p4: Point = Point {
		id: 4,
		vertex: vec![0, 0, 0],
		color: vec![128, 128, 128]
	};


	let line = MeshUnit::Line(vec![&p1, &p2]);
	let tri = MeshUnit::Triangle(vec![&p1, &p2, &p3]);
	let quad = MeshUnit::Quadrilateral(vec![&p1, &p2, &p3, &p4]);

	println!("{:#?}", line);
	println!("{:#?}", tri);
	println!("{:#?}", quad);
	

}