mod task1;
mod task2;
mod task3;
mod task4;

#[tokio::main]
async fn main() {
    println!("=== 任务一：SQLite 用户注册系统 ===");
    if let Err(e) = task1::run().await {
        eprintln!("任务一失败: {}", e);
    }

    println!("\n=== 任务二：MySQL 日志记录系统 ===");
    if let Err(e) = task2::run().await {
        eprintln!("任务二失败: {}", e);
    }

    println!("\n=== 任务三：PostgreSQL 商品库存管理 ===");
    if let Err(e) = task3::run().await {
        eprintln!("任务三失败: {}", e);
    }

    println!("\n=== 任务四：MySQL 数据统计报表 ===");
    if let Err(e) = task4::run().await {
        eprintln!("任务四失败: {}", e);
    }

    println!("\n所有任务执行完毕。");
}
