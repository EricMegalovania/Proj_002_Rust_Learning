use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读入一行字符串");
    let n: u32 = input.trim().parse().expect("一个 i32 整数");
    let mut ans: f64 = 0.0;
    for x in 1..=n {
        ans += (1.0 as f64) / (x as f64 * x as f64);
    }
    println!("前 {} 项的和是 {:.12}", n, ans);
}
