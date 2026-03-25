use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读入一行字符串");
    let score_100: i8 = input.trim().parse().expect("一个 i8 整数");
    let score_5: i8 = match score_100 {
        0..=59 => 1,
        60..=69 => 2,
        70..=79 => 3,
        80..=89 => 4,
        90..=100 => 5,
        _ => panic!("输入的成绩需要在 0 ~ 100 之内"),
    };
    println!("百分制成绩 {} 对应五分制成绩 {}", score_100, score_5);
}
