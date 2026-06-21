use sqlx::mysql::MySqlPool;
use sqlx::Row;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let pool = MySqlPool::connect("mysql://user_rshw03:123123@127.0.0.1:3306/rust_hw03").await?;
    println!("已连接 MySQL: rust_hw03");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS app_logs (
            log_id INT AUTO_INCREMENT PRIMARY KEY,
            log_level ENUM('DEBUG','INFO','WARN','ERROR') NOT NULL,
            content TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4",
    )
    .execute(&pool)
    .await?;
    println!("已创建/确认 app_logs 表");

    sqlx::query("TRUNCATE TABLE app_logs")
        .execute(&pool)
        .await?;

    let logs = [
        ("INFO", "系统启动完成"),
        ("DEBUG", "正在加载配置文件"),
        ("WARN", "磁盘使用率超过80%"),
        ("ERROR", "连接超时，重试中"),
        ("ERROR", "数据库查询失败：权限不足"),
        ("INFO", "健康检查通过"),
    ];

    println!("\n插入日志数据:");
    for (level, content) in &logs {
        match sqlx::query("INSERT INTO app_logs (log_level, content) VALUES (?, ?)")
            .bind(level)
            .bind(content)
            .execute(&pool)
            .await
        {
            Ok(r) => println!("  插入成功: [{}] {} (id={})", level, content, r.last_insert_id()),
            Err(e) => println!("  插入失败: {}", e),
        }
    }

    println!("\n查询最近24小时的 ERROR 级别日志:");
    let rows = sqlx::query(
        "SELECT log_id, log_level, content, created_at
         FROM app_logs
         WHERE log_level = 'ERROR' AND created_at >= NOW() - INTERVAL 1 DAY
         ORDER BY created_at DESC",
    )
    .fetch_all(&pool)
    .await?;

    if rows.is_empty() {
        println!("  无符合条件的日志");
    } else {
        for row in &rows {
            let id: i32 = row.get("log_id");
            let level: String = row.get("log_level");
            let content: String = row.get("content");
            let created_at: chrono::NaiveDateTime = row.get("created_at");
            println!(
                "  [{}] {} | {} | {}",
                id, created_at, level, content
            );
        }
    }

    pool.close().await;
    Ok(())
}
