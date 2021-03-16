fn main() {
    let a: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let b = [2.2; 10];

    println!("{:?}, Length: {}", a, a.len());
    println!("{:?}, Length: {}", b, b.len());

    // Slicing: data pointer based method

    let slice_a: &[i32] = &a[2..5];
    println!("{:?}, Length: {}", slice_a, slice_a.len());
}