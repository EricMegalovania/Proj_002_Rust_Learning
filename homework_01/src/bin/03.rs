use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读入一行字符串");
    let n: u32 = input.trim().parse().expect("一个 i32 整数");
    let mut a: Vec<Vec<u32>> = vec![vec![0; n as usize]; n as usize];
    a[0][0] = 1;
    for i in 0..(n - 1) as usize {
        for j in 0..(i + 1) {
            a[i + 1][j] += a[i][j];
            a[i + 1][j + 1] += a[i][j];
            print!("{} ", a[i][j])
        }
        println!();
    }
    for i in 0..(n) as usize {
        print!("{} ", a[(n - 1) as usize][i])
    }
    println!();
}
