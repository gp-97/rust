/// Case 1 - Return value check return value

fn divide(x: f32, y: f32) -> Option<f32> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn case_one() {
    let x = 10.0;
    let y = 120.0;
    let res = divide(x, y);
    match res {
        Some(v) => println!("{}", v),
        None => println!("[ERROR]: Division by zero"),
    };
}

/// Case 2 - Optional value return

struct Course {
    name: String,
    level: Option<String>,
}

fn case_two() {
    let c1 = Course {
        name: String::from("Rust"),
        level: Some(String::from("Beginner")),
    };
    let c2 = Course {
        name: String::from("C++"),
        level: None,
    };
    println!(
        "Course 1 : name: {}, level: {}",
        c1.name,
        c1.level.unwrap_or("Unknown Level".to_string())
    );
    println!(
        "Course 2 : name: {}, level: {}",
        c2.name,
        c2.level.unwrap_or("Unknown Level".to_string())
    );
}

/// Case 3 - Array/String/Vector index out-of-bounds exception handling

fn case_three() {
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    match vec.iter().nth(20) {
        Some(n) => println!("{}", n),
        None => println!("[ERROR]: Index out of bounds exception"),
    };
}

fn main() {
    case_one();
    case_two();
    case_three();
}
