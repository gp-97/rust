use std::fmt::Display; // Display trait

fn concatenate<T: Display>(t: T, s: T) {
    let conc = format!("{}{}", t, s);
    println!("{}", conc);
}

fn main() {
    let t = 10;
    let s = 20;
    concatenate(t, s);

    let t = "a";
    let s = "b";
    concatenate(t, s);
}
