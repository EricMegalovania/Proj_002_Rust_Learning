use sqlx::sqlite::SqlitePool;
use sqlx::Row;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePool::connect("sqlite://users.db?mode=rwc").await?;
    println!("已连接 SQLite: users.db");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL,
            created_at TEXT DEFAULT (datetime('now','localtime'))
        )",
    )
    .execute(&pool)
    .await?;
    println!("已创建/确认 users 表");

    let test_users = [
        ("alice", "pass123"),
        ("bob", "pass456"),
        ("charlie", "pass789"),
        ("diana", "pass000"),
        ("eve", "pass111"),
    ];

    println!("\n插入测试数据:");
    for (name, pass) in &test_users {
        match sqlx::query("INSERT OR IGNORE INTO users (username, password) VALUES (?, ?)")
            .bind(name)
            .bind(pass)
            .execute(&pool)
            .await
        {
            Ok(r) => {
                if r.rows_affected() > 0 {
                    println!("  插入成功: {}", name);
                } else {
                    println!("  已存在跳过: {}", name);
                }
            }
            Err(e) => println!("  插入失败 {}: {}", name, e),
        }
    }

    println!("\n按用户名查询 'alice':");
    let row = sqlx::query("SELECT id, username, password, created_at FROM users WHERE username = ?")
        .bind("alice")
        .fetch_optional(&pool)
        .await?;

    match row {
        Some(r) => {
            let id: i64 = r.get("id");
            let username: String = r.get("username");
            let password: String = r.get("password");
            let created_at: String = r.get("created_at");
            println!(
                "  查询结果: id={}, username={}, password={}, created_at={}",
                id, username, password, created_at
            );
        }
        None => println!("  未找到用户 'alice'"),
    }

    pool.close().await;
    Ok(())
}
