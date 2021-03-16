/// Passing and returning structs to & from functions
#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
    branch: String,
    cgpa: f32,
}

fn update_cg(s: &mut Student) {
    s.cgpa += 0.05;
}

fn update_age(mut s: Student) -> Student {
    s.age += 1;
    return s;
}

fn main() {
    let mut mahesh = Student {
        name: String::from("Mahesh"),
        age: 22,
        branch: String::from("EEE"),
        cgpa: 8.75,
    };
    println!("{:#?}", mahesh);
    update_cg(&mut mahesh);
    println!("{:#?}", mahesh);
    update_age(mahesh); // <-------------------------------------------------<
    //                                                                       |
    // println!("{:#?}", mahesh); Can't be done because value already moved--^
    let suresh = Student {
        name: String::from("Suresh"),
        age: 23,
        branch: String::from("ME"),
        cgpa: 9.75,
    };
    let suresh = update_age(suresh);
    println!("{:#?}", suresh);
}
