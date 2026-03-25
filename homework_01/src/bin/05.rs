use rand::Rng;

fn gen_op(operator: i8) -> fn(i16, i16) -> i16 {
    return match operator {
        0 => |x, y| x + y,
        1 => |x, y| x - y,
        2 => |x, y| x * y,
        3 => |x, y| {
            if y == 0 || x % y != 0 {
                return -1;
            } else {
                return x / y;
            }
        },
        _ => panic!("Invalid operator: must be 0, 1, 2 or 3"),
    };
}

fn gen_prob() -> (String, i8) {
    let mut rng = rand::thread_rng();
    let operator: i8 = rng.gen_range(0..4);
    // 0: +, 1: -, 2: *, 3: /
    let op = gen_op(operator);
    let op_symbol = "+-*/".chars().nth(operator as usize).unwrap();
    loop {
        let x: i16 = rng.gen_range(0..=20);
        let y: i16 = rng.gen_range(0..=20);
        let res: i16 = op(x, y);
        if res <= 0 || res > 20 {
            continue;
        }
        return (format!("{} {} {}", x, op_symbol, y), res as i8);
    }
}

fn main() {
    let (problem, answer) = gen_prob();
    println!("{} = {}", problem, answer)
}
