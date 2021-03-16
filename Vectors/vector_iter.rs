fn main() {
    let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
    let idx = vec.iter().position(|&e|e==30).unwrap();
    println!("{}", idx);

    let mut vec2: Vec<i32> = vec![];
    for i in 0..15 {
        vec2.push(i+15);
    }
    println!("{:?}", vec2);
    let mut index = 0;
    for j in vec2.iter() {
        print!("arr[{}] = {} ", index, j);
        index += 1;
    }

    let mut vec3 = vec![];
    for i in 0..15 {
        vec3.push(i+10);
    }
    for i in vec3.iter_mut() {
        *i *= 10;
    }
    for i in vec3.iter_mut() {
        print!("{} ", i);
    }
    println!();
}