fn main() {
    let au: u8 = 234;
    let bu: u32 = 1232;
    let cu: u64 = 324234;
    let du: u128 = 24234;
    let eu: usize = 34;

    let ai: i8 = 23;
    let bi: i32 = 1232;
    let ci: i64 = 324234;
    let di: i128 = 24234;
    let ei: isize = 34;

    let af: f32 = 24.234;
    let bf: f64 = 234.23434;

    let ab: bool = false;

    let achar: char = 'a';

    let astring: &str = "Rust";

    println!(
        "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        au, bu, cu, du, eu, ai, bi, ci, di, ei, af, bf, ab, achar, astring
    );
}
