use std::io;

struct Circle {
    radius: f32,
}

struct Rectangle {
    length: f32,
    breadth: f32,
}

trait Shape2D {
    fn calc_area(&self) -> f32;
    fn calc_perimeter(&self) -> f32;
}

impl Shape2D for Circle {
    fn calc_area(&self) -> f32 {
        (355.0 / 113.0) * f32::powf(self.radius, 2.0)
    }
    fn calc_perimeter(&self) -> f32 {
        2.0 * (355.0 / 113.0) * self.radius
    }
}

impl Shape2D for Rectangle {
    fn calc_area(&self) -> f32 {
        self.length * self.breadth
    }
    fn calc_perimeter(&self) -> f32 {
        2.0 * (self.length + self.breadth)
    }
}

fn get_user_input() -> (f32, f32, f32) {
    println!("Enter radius");
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Enter 32-bit floating number");
    inp = inp.trim().to_string();
    let radius = inp.parse::<f32>().unwrap();

    println!("Enter length");
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Enter 32-bit floating number");
    inp = inp.trim().to_string();
    let length = inp.parse::<f32>().unwrap();

    println!("Enter breadth");
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Enter 32-bit floating number");
    inp = inp.trim().to_string();
    let breadth = inp.parse::<f32>().unwrap();

    (radius, length, breadth)
}

fn main() {
    let user_inp = get_user_input();
    let circle = Circle { radius: user_inp.0 };
    let rectangle = Rectangle {
        length: user_inp.1,
        breadth: user_inp.2,
    };
    println!("Circle area: {} square units", circle.calc_area());
    println!("Circle perimeter: {} units", circle.calc_perimeter());
    println!("Rectangle area: {} square units", rectangle.calc_area());
    println!("Rectangle perimeter: {} units", rectangle.calc_perimeter());
}
