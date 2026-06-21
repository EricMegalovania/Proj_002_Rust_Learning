use sqlx::mysql::MySqlPool;
use sqlx::Row;

#[allow(dead_code)]
struct Order {
    order_id: i32,
    amount: f64,
    created_at: chrono::NaiveDateTime,
}

struct DailySalesReport {
    order_date: chrono::NaiveDate,
    order_count: i64,
    total_sales: f64,
    avg_order_value: f64,
}

struct OrderRepo {
    pool: MySqlPool,
}

impl OrderRepo {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let pool =
            MySqlPool::connect("mysql://user_rshw03:123123@127.0.0.1:3306/rust_hw03").await?;
        println!("  已连接 MySQL: rust_hw03");
        Ok(Self { pool })
    }

    async fn create_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS orders (
                order_id INT AUTO_INCREMENT PRIMARY KEY,
                amount DECIMAL(10,2) NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4",
        )
        .execute(&self.pool)
        .await?;
        println!("  orders 表已就绪");
        Ok(())
    }

    async fn truncate(&self) -> Result<(), sqlx::Error> {
        sqlx::query("TRUNCATE TABLE orders")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn insert_with_days_ago(
        &self,
        amount: f64,
        days_ago: i64,
    ) -> Result<u64, sqlx::Error> {
        let r = sqlx::query(
            "INSERT INTO orders (amount, created_at) VALUES (?, NOW() - INTERVAL ? DAY)",
        )
        .bind(amount)
        .bind(days_ago)
        .execute(&self.pool)
        .await?;
        Ok(r.last_insert_id())
    }

    async fn create_report_view(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE OR REPLACE VIEW daily_sales_report AS
             SELECT
                 DATE(created_at) AS order_date,
                 COUNT(*) AS order_count,
                 CAST(COALESCE(SUM(amount), 0.00) AS DOUBLE) AS total_sales,
                 CAST(ROUND(COALESCE(AVG(amount), 0.00), 2) AS DOUBLE) AS avg_order_value
             FROM orders
             GROUP BY DATE(created_at)
             ORDER BY order_date DESC",
        )
        .execute(&self.pool)
        .await?;
        println!("  视图 daily_sales_report 已创建/更新");
        Ok(())
    }

    async fn query_report(
        &self,
        from: &str,
        to: &str,
    ) -> Result<Vec<DailySalesReport>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT order_date, order_count, total_sales, avg_order_value
             FROM daily_sales_report
             WHERE order_date BETWEEN ? AND ?
             ORDER BY order_date",
        )
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|r| DailySalesReport {
                order_date: r.get("order_date"),
                order_count: r.get("order_count"),
                total_sales: r.get("total_sales"),
                avg_order_value: r.get("avg_order_value"),
            })
            .collect())
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 测试1：连接数据库并建表 ===");
    let repo = OrderRepo::new().await?;
    repo.create_table().await?;

    println!("\n=== 测试2：清空旧数据 ===");
    repo.truncate().await?;
    println!("  已清空 orders 表");

    println!("\n=== 测试3：插入 8 条跨天订单 ===");
    let orders = [
        (199.00, 0),
        (299.50, 0),
        (89.90, 0),
        (450.00, 1),
        (129.00, 1),
        (599.00, 2),
        (79.00, 2),
        (349.00, 2),
    ];
    for (amount, days_ago) in &orders {
        let id = repo.insert_with_days_ago(*amount, *days_ago).await?;
        println!("  ¥{:>8.2} ({:>2}天前) -> order_id={}", amount, days_ago, id);
    }

    println!("\n=== 测试4：创建销售报表视图 ===");
    repo.create_report_view().await?;

    println!("\n=== 测试5：查询 2026-06-18 至 2026-06-21 的每日销售报表 ===");
    let report = repo.query_report("2026-06-18", "2026-06-21").await?;
    if report.is_empty() {
        println!("  该日期范围内无订单数据");
    } else {
        println!(
            "  {:<12} {:<8} {:<12} {:<14}",
            "日期", "订单数", "销售总额", "平均客单价"
        );
        println!("  {:-<48}", "");
        for r in &report {
            println!(
                "  {:<12} {:<8} ¥{:<11.2} ¥{:<13.2}",
                r.order_date, r.order_count, r.total_sales, r.avg_order_value
            );
        }
    }

    repo.pool.close().await;
    Ok(())
}
