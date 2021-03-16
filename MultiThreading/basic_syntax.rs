use std::thread;

fn main() {
    thread::spawn(move || {
        println!("Thread no. {:?}", thread::current());
    });
}
