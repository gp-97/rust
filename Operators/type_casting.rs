/*
    'as' -> keyword used for typecasting.

    int<->float
    char<-!- int <-!-str
    char<-!->&str
*/

#[allow(unused_variables, unused_mut)]

fn main() {
    let a: i32 = 24;
    let b = (a as f64) / 3.2;
    println!("a = {}, b = {}", a, b);

    let a: char = 'a';
    let mut b: i32 = a as i32;
    b += 25;
    let c: char = b as char;
    println!("a = {}, b = {}, c = {}", a, b, c);
}
