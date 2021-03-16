#[allow(unused_variables, unused_mut)]
fn main() {
    let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7 ,8 ,9 ,10];
    vec.push(11);
    println!("{:?}", vec);
    let pop_val = vec.pop();
    println!("{:?}", vec);
    match pop_val {
        Some(x) => println!("{}", x),
        None => println!("No value returned by pop fn")
    };
    let mut vec2 = Vec::from(vec);
    let rem_val = vec2.remove(1);
    println!("{}", rem_val);
    println!("{:?}", vec2);

}