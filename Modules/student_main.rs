mod student;

fn main() {
    let jd = student::Student::new(
        String::from("John Doe"),
        23,
        String::from("EEE"),
        9.45,
        String::from("Male"),
    );

    student::display_student(&jd);
}
