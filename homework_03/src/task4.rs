use sqlx::mysql::MySqlPool;
use sqlx::Row;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let pool = MySqlPool::connect("mysql://user_rshw03:123123@127.0.0.1:3306/rust_hw03").await?;
    println!("已连接 MySQL: rust_hw03");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS orders (
            order_id INT AUTO_INCREMENT PRIMARY KEY,
            amount DECIMAL(10,2) NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4",
    )
    .execute(&pool)
    .await?;
    println!("已创建/确认 orders 表");

    sqlx::query("TRUNCATE TABLE orders")
        .execute(&pool)
        .await?;

    let orders = [
        ("NOW()", 199.00),
        ("NOW()", 299.50),
        ("NOW()", 89.90),
        ("NOW() - INTERVAL 1 DAY", 450.00),
        ("NOW() - INTERVAL 1 DAY", 129.00),
        ("NOW() - INTERVAL 2 DAY", 599.00),
        ("NOW() - INTERVAL 2 DAY", 79.00),
        ("NOW() - INTERVAL 2 DAY", 349.00),
    ];

    println!("\n插入测试订单:");
    for (date_expr, amount) in &orders {
        let sql = format!(
            "INSERT INTO orders (amount, created_at) VALUES ({}, {})",
            amount, date_expr
        );
        match sqlx::query(&sql).execute(&pool).await {
            Ok(r) => println!(
                "  插入成功: ¥{:.2} (order_id={})",
                amount,
                r.last_insert_id()
            ),
            Err(e) => println!("  插入失败: {}", e),
        }
    }

    println!("\n创建/更新视图 daily_sales_report:");
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
    .execute(&pool)
    .await?;
    println!("  视图已创建/更新");

    println!("\n查询 2026-06-18 至 2026-06-21 的每日销售报表:");
    let rows = sqlx::query(
        "SELECT order_date, order_count, total_sales, avg_order_value
         FROM daily_sales_report
         WHERE order_date BETWEEN ? AND ?
         ORDER BY order_date",
    )
    .bind("2026-06-18")
    .bind("2026-06-21")
    .fetch_all(&pool)
    .await?;

    if rows.is_empty() {
        println!("  该日期范围内无订单数据");
    } else {
        println!("  {:<12} {:<10} {:<12} {:<14}", "日期", "订单数", "销售总额", "平均客单价");
        println!("  {:-<48}", "");
        for row in &rows {
            let date: chrono::NaiveDate = row.get("order_date");
            let count: i64 = row.get("order_count");
            let total: f64 = row.get("total_sales");
            let avg: f64 = row.get("avg_order_value");
            println!(
                "  {:<12} {:<10} ¥{:<11.2} ¥{:<13.2}",
                date, count, total, avg
            );
        }
    }

    pool.close().await;
    Ok(())
}
