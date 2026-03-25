[TOC]

<div style="page-break-after: always;"></div>

## 安装 Rust

Windows：<https://beatai.org/rust-course/first-try/installation#在-windows-上安装-rustup>

- 用 Git-Bash 配合 `x86_64-pc-windows-gnu` 的方式来安装

## 创建项目

```bash
cargo new PROJECT_NAME --vcs none
```

`PROJECT_NAME` 不能以**数字**开头

`--vcs none` 指定不使用任何版本控制系统

`--vcs git` 指定使用 git 作为版本控制系统

### 创建算法竞赛项目

```plain
my_contest/
├── Cargo.toml
└── src/
    ├── lib.rs          # 共享的辅助函数
    └── bin/
        ├── problem_a.rs
        ├── problem_b.rs
        └── problem_c.rs
```

用类似这样的结构来管理一场比赛

比如对于 `homework_01`，可以这样子创建：

```bash
cargo new homework_01 --vcs none
cd ./homework_01/src
mkdir bin
touch ./bin/03.rs
```

测试代码就用：

```bash
cargo run --bin 03
```

为了方便起见，我在 `~/.bashrc` 中定义了这个命令

```bash
rsl(){
	cargo run --bin "$1"
}
```

这样用 `rsl 03` 就可以运行了

## 输入

```rust
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读入一行字符串");
    let n: i32 = input.trim().parse().expect("一个 i32 整数");
    println!("{}", n);
}
```

我们从**完全小白**的角度来解释一下语法：

`mut`：表示变量可以被修改，在 Rust 中，变量默认是不能修改的（这是 Rust 的设计哲学）

`String::new()`：创建一个新的空字符串

`let mut input = String::new()`：将 `input` 绑定到这个字符串，并标记为可变

`io::stdin().read_line(&mut input)`：从标准输入中读取一行，**追加**到 `input` 中

- `&mut`：`&` 表示引用，`mut` 表示可变，合在一起，就是允许函数修改 `input` 的内容

`expect("SOME_STRING")`：`read_line()` 返回 `Result` 类型

- 成功：继续运行
- 失败：打印 `expect()` 里的字符串并终止程序

`input.trim()`：移除字符串首尾的空白字符（因为 `read_line()` 包含用户按下的回车键 `\n`）

`parse()`：将字符串解析为指定类型

- 这里通过 `let n: i32` 的 `i32` 这个**类型注解**，告诉编译器要解析为 32 位整数

## 数组与动态数组

有点类似 Python 定义数组的方法

### 数组

```rust
// 定义 3x4 的二维数组（3行4列）
// 数组的每一维都必须是常量，大小固定
let mut matrix: [[i32; 4]; 3] = [[0; 4]; 3];
```

### 动态数组

```rust
// n 是一个输入的整数
let mut a: Vec<Vec<u32>> = vec![vec![0; n as usize]; n as usize];
```

## 循环与输出（杨辉三角）

不想解释了，我们放个**杨辉三角形**的例子吧（homework_01_03）：

```rust
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
```

值得注意的点有：

- 下标必须是 `usize` 类型！！
  - 类型转换用 `variable as usize`
- `print!()` 和 `println!()` 都是输出宏，类似 Python 的 fmt 输出

## 函数与闭包

### 函数

```rust
fn add(i: i32, j: i32) -> i32 {
    i + j
}
```

Rust 的函数的定义很像 C++ 的 lambda 表达式，但是**可以省略** `return`

（但是我不习惯省略，感觉会增加阅读负担）

### 闭包

就是类似于其他语言的 lambda 表达式的概念，我们把上面这个例子的 `add()` 转成闭包写法

```rust
fn main() {
    let add = |x: i32, y: i32| -> i32 {
        return x + y;
    };
}
```

不过闭包一般还是在**匿名函数**中用的更多一些

### homework_01_04

```rust
use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("输入一行字符串");
    s = String::from(s.trim());
    // 或者 s = s.trim().to_string(); 二者是等价的
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
```

`String.chars().map(...).collect()` 算是个固定写法吧，用的多了肯定就记住了 mua

## 随机数

在 `Cargo.toml` 中添加：

```toml
[dependencies]
rand = "0.8"
```

使用随机数的方法：

```rust
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();   # 定义一个随机数
    let mut rd = rng.gen_range(0..=3);  # 生成一个 [0,3] 内的随机整数
    rd = rng.gen_range(0..3);           # 生成一个 [0,3) 内的随机整数
}
```

## match 的用法
