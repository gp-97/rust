#[derive(Debug)]

struct Person {
    name: String,
    age: u8
}

impl Person {
    fn get_age(&self) -> u8{
        self.age
    }
}

fn main() {
    let p1 = Person {
        name: String::from("p1"),
        age: 23
    };
    println!("{}", p1.get_age());
    println!("{:#?}", p1);
}