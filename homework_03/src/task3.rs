use sqlx::postgres::PgPool;
use sqlx::Row;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPool::connect("postgres://user_rshw03:123123@127.0.0.1:5432/rust_hw03").await?;
    println!("已连接 PostgreSQL: rust_hw03");

    sqlx::query("DROP TABLE IF EXISTS products CASCADE")
        .execute(&pool)
        .await?;

    sqlx::query(
        "CREATE TABLE products (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL UNIQUE,
            stock INT NOT NULL DEFAULT 0,
            updated_at TIMESTAMP DEFAULT NOW()
        )",
    )
    .execute(&pool)
    .await?;
    println!("已创建 products 表");

    let products = [
        ("笔记本", 5),
        ("鼠标", 20),
        ("键盘", 8),
        ("显示器", 3),
        ("耳机", 12),
    ];

    println!("\n插入测试商品:");
    for (name, stock) in &products {
        match sqlx::query(
            "INSERT INTO products (name, stock) VALUES ($1, $2)",
        )
        .bind(name)
        .bind(stock)
        .execute(&pool)
        .await
        {
            Ok(r) => {
                if r.rows_affected() > 0 {
                    println!("  插入成功: {} (库存: {})", name, stock);
                } else {
                    println!("  已存在跳过: {}", name);
                }
            }
            Err(e) => println!("  插入失败 {}: {}", name, e),
        }
    }

    println!("\n库存扣减: 购买 3 件「笔记本」");
    let result = sqlx::query(
        "UPDATE products SET stock = stock - $1, updated_at = NOW() WHERE name = $2 AND stock >= $1",
    )
    .bind(3)
    .bind("笔记本")
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        println!("  扣减失败: 库存不足或商品不存在");
    } else {
        let row = sqlx::query("SELECT name, stock FROM products WHERE name = $1")
            .bind("笔记本")
            .fetch_one(&pool)
            .await?;
        let name: String = row.get("name");
        let stock: i32 = row.get("stock");
        println!("  扣减成功: {} 剩余库存 {}", name, stock);
    }

    println!("\n库存扣减: 购买 5 件「显示器」(库存不足场景)");
    let result = sqlx::query(
        "UPDATE products SET stock = stock - $1, updated_at = NOW() WHERE name = $2 AND stock >= $1",
    )
    .bind(5)
    .bind("显示器")
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        println!("  扣减失败: 库存不足或商品不存在");
    } else {
        println!("  扣减成功");
    }

    println!("\n查询库存低于 10 件的商品:");
    let rows = sqlx::query("SELECT id, name, stock, updated_at FROM products WHERE stock < 10 ORDER BY stock ASC")
        .fetch_all(&pool)
        .await?;

    if rows.is_empty() {
        println!("  所有商品库存充足");
    } else {
        for row in &rows {
            let id: i32 = row.get("id");
            let name: String = row.get("name");
            let stock: i32 = row.get("stock");
            let updated_at: chrono::NaiveDateTime = row.get("updated_at");
            println!("  id={} {} 库存={} 更新时间={}", id, name, stock, updated_at);
        }
    }

    pool.close().await;
    Ok(())
}
