use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), std::io::Error> {
    let path = "file.txt";
    let file = match File::open(path) {
        Ok(f) => f,
        Err(r) => return Err(r),
    };
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    match buf_reader.read_to_string(&mut content) {
        Ok(_) => Ok(&content),
        Err(r) => Err(r),
    };
    println!("{}", content);
    Ok(())
}
