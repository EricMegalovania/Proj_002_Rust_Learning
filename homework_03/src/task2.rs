use sqlx::mysql::MySqlPool;
use sqlx::Row;

enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

struct AppLog {
    #[allow(dead_code)]
    log_id: i32,
    log_level: String,
    content: String,
    created_at: chrono::NaiveDateTime,
}

impl std::fmt::Display for AppLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} | {} | {}",
            self.log_id, self.created_at, self.log_level, self.content
        )
    }
}

struct LogRepo {
    pool: MySqlPool,
}

impl LogRepo {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let pool =
            MySqlPool::connect("mysql://user_rshw03:123123@127.0.0.1:3306/rust_hw03").await?;
        println!("  已连接 MySQL: rust_hw03");
        Ok(Self { pool })
    }

    async fn create_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS app_logs (
                log_id INT AUTO_INCREMENT PRIMARY KEY,
                log_level ENUM('DEBUG','INFO','WARN','ERROR') NOT NULL,
                content TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4",
        )
        .execute(&self.pool)
        .await?;
        println!("  app_logs 表已就绪");
        Ok(())
    }

    async fn truncate(&self) -> Result<(), sqlx::Error> {
        sqlx::query("TRUNCATE TABLE app_logs")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn insert(&self, level: &LogLevel, content: &str) -> Result<u64, sqlx::Error> {
        let r = sqlx::query("INSERT INTO app_logs (log_level, content) VALUES (?, ?)")
            .bind(level.as_str())
            .bind(content)
            .execute(&self.pool)
            .await?;
        Ok(r.last_insert_id())
    }

    async fn find_errors_24h(&self) -> Result<Vec<AppLog>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT log_id, log_level, content, created_at
             FROM app_logs
             WHERE log_level = 'ERROR' AND created_at >= NOW() - INTERVAL 1 DAY
             ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|r| AppLog {
                log_id: r.get("log_id"),
                log_level: r.get("log_level"),
                content: r.get("content"),
                created_at: r.get("created_at"),
            })
            .collect())
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 测试1：连接数据库并建表 ===");
    let repo = LogRepo::new().await?;
    repo.create_table().await?;

    println!("\n=== 测试2：清空旧数据 ===");
    repo.truncate().await?;
    println!("  已清空 app_logs 表");

    println!("\n=== 测试3：插入不同级别的日志 ===");
    let test_logs: [(&LogLevel, &str); 6] = [
        (&LogLevel::Info, "系统启动完成"),
        (&LogLevel::Debug, "正在加载配置文件"),
        (&LogLevel::Warn, "磁盘使用率超过80%"),
        (&LogLevel::Error, "连接超时，重试中"),
        (&LogLevel::Error, "数据库查询失败：权限不足"),
        (&LogLevel::Info, "健康检查通过"),
    ];
    for (level, content) in &test_logs {
        let id = repo.insert(level, content).await?;
        println!("  [{:5}] {} (log_id={})", level.as_str(), content, id);
    }

    println!("\n=== 测试4：查询最近24小时的 ERROR 日志 ===");
    let errors = repo.find_errors_24h().await?;
    if errors.is_empty() {
        println!("  无符合条件的日志");
    } else {
        for log in &errors {
            println!("  {}", log);
        }
    }

    repo.pool.close().await;
    Ok(())
}
