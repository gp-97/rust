fn sum(arr: [i32; 5]) {
    let mut sum = 0;
    for i in 0..arr.len() {
        sum += arr[i];
    }
    println!("Array sum: {}", sum);
}

fn cummulative_sum(arr: &mut [i32; 5]) {
    for i in 1..arr.len() {
        arr[i] += arr[i-1];
    }
}

fn main() {
    let mut arr: [i32; 5] = [0, 1, 2, 3, 4];
    sum(arr);
    println!("{:?}", arr);
    cummulative_sum(&mut arr);
    println!("{:?}", arr);
}