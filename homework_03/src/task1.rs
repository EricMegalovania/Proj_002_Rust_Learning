use sqlx::sqlite::SqlitePool;
use sqlx::Row;

struct User {
    #[allow(dead_code)]
    id: i64,
    username: String,
    password: String,
    created_at: String,
}

struct UserRepo {
    pool: SqlitePool,
}

impl UserRepo {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let pool = SqlitePool::connect("sqlite://users.db?mode=rwc").await?;
        println!("  已连接 SQLite: users.db");
        Ok(Self { pool })
    }

    async fn create_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                password TEXT NOT NULL,
                created_at TEXT DEFAULT (datetime('now','localtime'))
            )",
        )
        .execute(&self.pool)
        .await?;
        println!("  users 表已就绪");
        Ok(())
    }

    async fn insert(&self, username: &str, password: &str) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("INSERT OR IGNORE INTO users (username, password) VALUES (?, ?)")
            .bind(username)
            .bind(password)
            .execute(&self.pool)
            .await?;
        Ok(r.rows_affected() > 0)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query(
            "SELECT id, username, password, created_at FROM users WHERE username = ?",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            username: r.get("username"),
            password: r.get("password"),
            created_at: r.get("created_at"),
        }))
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 测试1：创建数据库连接并建表 ===");
    let repo = UserRepo::new().await?;
    repo.create_table().await?;

    println!("\n=== 测试2：插入 5 条用户数据 ===");
    let test_users = [
        ("alice", "pass123"),
        ("bob", "pass456"),
        ("charlie", "pass789"),
        ("diana", "pass000"),
        ("eve", "pass111"),
    ];
    for (name, pass) in &test_users {
        let inserted = repo.insert(name, pass).await?;
        println!(
            "  {} -> {}",
            name,
            if inserted { "插入成功" } else { "已存在跳过" }
        );
    }

    println!("\n=== 测试3：按用户名查询 ===");
    match repo.find_by_username("alice").await? {
        Some(u) => println!(
            "  查询成功: id={}, username={}, password={}, created_at={}",
            u.id, u.username, u.password, u.created_at
        ),
        None => println!("  未找到用户 alice"),
    }

    println!("\n=== 测试4：查询不存在的用户 ===");
    match repo.find_by_username("nobody").await? {
        Some(_) => println!("  意外找到用户"),
        None => println!("  正确返回 None,未找到用户 nobody"),
    }

    repo.pool.close().await;
    Ok(())
}
