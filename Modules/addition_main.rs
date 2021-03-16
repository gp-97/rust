mod addition;

fn main() {
    let res = addition::add(5, 6);
    addition::display(res.to_string());
}