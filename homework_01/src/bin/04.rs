use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("输入一行字符串");
    s = String::from(s.trim());
    let s: String = s
        .chars()
        .map(|c| -> char {
            if !c.is_ascii_alphabetic() {
                return c;
            } else if c.is_ascii_lowercase() {
                return c.to_ascii_uppercase();
            } else {
                return c.to_ascii_lowercase();
            }
        })
        .collect();
    println!("{}", s);
}
