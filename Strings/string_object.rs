fn main() {
    let mut s: String = String::new();
    s.push('A');
    println!("{}", s);
    s.push_str("BCDEF");
    println!("{}", s);

    let sl: &str = &s[2..4];
    println!("{:?}", sl);

    let sl_obj = sl.to_string();
    println!("{:?}", sl_obj);

    println!("{}", s.starts_with("AB"));
    println!("{}", s.contains("DE"));
    println!("{}", s.ends_with("AB"));

    let sent = String::from("Once upon a time.");
    for token in sent.split_whitespace() {
        println!("{} ", token);
    }
    for ch in sent.chars() {
        println!("{}", ch);
    }
}