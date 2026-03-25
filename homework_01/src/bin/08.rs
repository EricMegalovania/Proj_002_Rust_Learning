use rand::Rng;
use std::io;

fn get_player_choice() -> u32 {
    loop {
        let mut player_input = String::new();
        io::stdin()
            .read_line(&mut player_input)
            .expect("读取输入失败");

        match player_input.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 3 {
                    println!("输入的数字需要在 1 ~ 3 之内，请重新输入");
                    continue;
                } else {
                    return num;
                }
            }
            Err(_) => {
                println!("请输入有效的数字 1、2 或 3");
                continue;
            }
        };
    }
}

fn main() {
    println!("石头剪刀布游戏！三局两胜制！");

    let mut player_score = 0;
    let mut computer_score = 0;
    let mut round = 1;
    let mut round_results = Vec::new(); // 用于记录每局结果

    // 游戏循环，直到任意一方达到2分或已进行3局
    while player_score < 2 && computer_score < 2 && round <= 3 {
        println!("\n--- 第 {} 局 ---", round);
        println!("请输入你的选择: 1=石头, 2=剪刀, 3=布");
        let player_choice = get_player_choice();
        let computer_choice: u32 = rand::thread_rng().gen_range(1..=3);

        // 将数字转换为手势名称
        let choice_name = |choice: u32| -> &'static str {
            match choice {
                1 => "石头",
                2 => "剪刀",
                3 => "布",
                _ => panic!("无效的选择"),
            }
        };

        let player_choice_name = choice_name(player_choice);
        let computer_choice_name = choice_name(computer_choice);

        println!("　你出了: {}", player_choice_name);
        println!("电脑出了: {}", computer_choice_name);

        // 判断胜负
        let (result, win) = match (player_choice, computer_choice) {
            (1, 2) | (2, 3) | (3, 1) => {
                player_score += 1;
                ("你赢了！", true)
            }
            (1, 1) | (2, 2) | (3, 3) => ("平局！", false),
            (1, 3) | (2, 1) | (3, 2) => {
                computer_score += 1;
                ("你输了！", false)
            }
            _ => ("无效的对决", false),
        };

        println!("本局结果: {}", result);
        println!("当前比分: 玩家 {} : {} 电脑", player_score, computer_score);

        // 记录本局结果
        round_results.push(format!(
            "第{}局: 你出了{}，电脑出了{}，{}",
            round, player_choice_name, computer_choice_name, result
        ));

        round += 1;
    }

    // 游戏结束，输出总结
    println!("\n=== 游戏结束 ===");
    println!("\n每局结果:");
    for result in &round_results {
        println!("  {}", result);
    }

    println!(
        "\n最终比分: 玩家 {} : {} 电脑",
        player_score, computer_score
    );

    if player_score > computer_score {
        println!("恭喜！你赢得了比赛！");
    } else if computer_score > player_score {
        println!("很遗憾，电脑赢得了比赛。");
    } else {
        println!("比赛平局！");
    }
}
