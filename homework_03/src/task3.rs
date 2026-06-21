use sqlx::postgres::PgPool;
use sqlx::Row;

struct Product {
    #[allow(dead_code)]
    id: i32,
    name: String,
    stock: i32,
    updated_at: chrono::NaiveDateTime,
}

struct ProductRepo {
    pool: PgPool,
}

impl ProductRepo {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let pool =
            PgPool::connect("postgres://user_rshw03:123123@127.0.0.1:5432/rust_hw03").await?;
        println!("  已连接 PostgreSQL: rust_hw03");
        Ok(Self { pool })
    }

    async fn create_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query("DROP TABLE IF EXISTS products CASCADE")
            .execute(&self.pool)
            .await?;

        sqlx::query(
            "CREATE TABLE products (
                id SERIAL PRIMARY KEY,
                name VARCHAR(100) NOT NULL UNIQUE,
                stock INT NOT NULL DEFAULT 0,
                updated_at TIMESTAMP DEFAULT NOW()
            )",
        )
        .execute(&self.pool)
        .await?;
        println!("  products 表已就绪");
        Ok(())
    }

    async fn insert(&self, name: &str, stock: i32) -> Result<bool, sqlx::Error> {
        let r =
            sqlx::query("INSERT INTO products (name, stock) VALUES ($1, $2) ON CONFLICT (name) DO NOTHING")
                .bind(name)
                .bind(stock)
                .execute(&self.pool)
                .await?;
        Ok(r.rows_affected() > 0)
    }

    async fn deduct_stock(&self, name: &str, quantity: i32) -> Result<bool, sqlx::Error> {
        let r = sqlx::query(
            "UPDATE products SET stock = stock - $1, updated_at = NOW() WHERE name = $2 AND stock >= $1",
        )
        .bind(quantity)
        .bind(name)
        .execute(&self.pool)
        .await?;
        Ok(r.rows_affected() > 0)
    }

    async fn get_stock(&self, name: &str) -> Result<i32, sqlx::Error> {
        let row = sqlx::query("SELECT stock FROM products WHERE name = $1")
            .bind(name)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.get("stock"))
    }

    async fn find_low_stock(&self, threshold: i32) -> Result<Vec<Product>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, name, stock, updated_at FROM products WHERE stock < $1 ORDER BY stock ASC",
        )
        .bind(threshold)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|r| Product {
                id: r.get("id"),
                name: r.get("name"),
                stock: r.get("stock"),
                updated_at: r.get("updated_at"),
            })
            .collect())
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 测试1：连接数据库并建表 ===");
    let repo = ProductRepo::new().await?;
    repo.create_table().await?;

    println!("\n=== 测试2：插入 5 件商品 ===");
    let products = [
        ("笔记本", 5),
        ("鼠标", 20),
        ("键盘", 8),
        ("显示器", 3),
        ("耳机", 12),
    ];
    for (name, stock) in &products {
        let inserted = repo.insert(name, *stock).await?;
        println!(
            "  {} (库存 {}) -> {}",
            name,
            stock,
            if inserted {
                "插入成功"
            } else {
                "已存在跳过"
            }
        );
    }

    println!("\n=== 测试3：正常库存扣减 — 购买 3 件「笔记本」 ===");
    let ok = repo.deduct_stock("笔记本", 3).await?;
    if ok {
        let remaining = repo.get_stock("笔记本").await?;
        println!("  扣减成功: 笔记本 剩余库存 {}", remaining);
    } else {
        println!("  扣减失败: 库存不足");
    }

    println!("\n=== 测试4：库存不足扣减 — 购买 5 件「显示器」(库存仅3) ===");
    let ok = repo.deduct_stock("显示器", 5).await?;
    if ok {
        println!("  扣减成功(意外)");
    } else {
        let remaining = repo.get_stock("显示器").await?;
        println!("  扣减被拒绝: 显示器 库存不足,当前库存为 {}", remaining);
    }

    println!("\n=== 测试5：查询库存低于 10 件的商品 ===");
    let low = repo.find_low_stock(10).await?;
    if low.is_empty() {
        println!("  所有商品库存充足");
    } else {
        for p in &low {
            println!(
                "  id={} {} 库存={} 更新时间={}",
                p.id, p.name, p.stock, p.updated_at
            );
        }
    }

    repo.pool.close().await;
    Ok(())
}
