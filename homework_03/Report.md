# Rust 第三次实验 — 结果分析

---

## 任务一：用户注册系统（SQLite）

### 实验要求

- 创建 SQLite 数据库 `users.db`
- 建立 `users` 表（id 主键、username 唯一、password、created_at 时间戳）
- 编写 Rust 程序实现用户注册功能，插入测试数据（至少 5 条）
- 实现按用户名查询用户信息的功能

### 实现要点

使用 `sqlx` 库连接 SQLite 数据库,通过 `CREATE TABLE IF NOT EXISTS` 建表,定义 `id INTEGER PRIMARY KEY AUTOINCREMENT`、`username TEXT UNIQUE NOT NULL`、`password TEXT NOT NULL`、`created_at TEXT DEFAULT (datetime('now','localtime'))`。

插入阶段使用 `INSERT OR IGNORE` 确保重复运行不会报错,共插入 5 条测试用户（alice / bob / charlie / diana / eve）。

查询阶段使用 `SELECT ... WHERE username = ?` 绑定参数查询指定用户,通过 `Row::get()` 逐字段提取结果。

### 运行结果

[运行任务一-SQLite建表与插入用户]

[运行任务一-按用户名查询]

### 结果分析

- `users.db` 在项目根目录自动生成,建表语句正确执行
- 5 条测试数据全部插入成功,重复运行时 `INSERT OR IGNORE` 正确跳过已存在记录
- 以 `alice` 为条件进行用户名查询,成功返回 id、username、password、created_at 四个字段,时间戳采用本地时间格式

---

## 任务二：日志记录系统（MySQL）

### 实验要求

- 连接本地 MySQL 数据库创建 `app_logs` 表
- 字段包含：log_id（自增）、log_level（ENUM）、content、created_at
- 编写日志写入函数，支持插入不同级别的日志
- 实现查询最近 24 小时的 ERROR 级别日志

### 实现要点

连接 MySQL 后使用 `CREATE TABLE IF NOT EXISTS` 建表,log_level 字段定义为 `ENUM('DEBUG','INFO','WARN','ERROR')`,log_id 使用 `INT AUTO_INCREMENT PRIMARY KEY`,created_at 使用 `DATETIME DEFAULT CURRENT_TIMESTAMP`。

插入了 6 条覆盖全部四个级别的日志(INFO × 2、DEBUG × 1、WARN × 1、ERROR × 2)。

查询使用 `WHERE log_level = 'ERROR' AND created_at >= NOW() - INTERVAL 1 DAY`,通过 `Row::get()` 提取 log_id(log_level/content/created_at,其中 created_at 映射为 `chrono::NaiveDateTime`。每次运行前 `TRUNCATE TABLE` 确保输出一致。

### 运行结果

[运行任务二-MySQL建表与日志写入]

[运行任务二-查询最近24小时ERROR日志]

### 结果分析

- `app_logs` 表创建成功,ENUM 类型字段正确约束了 log_level 的取值
- 6 条日志涵盖 DEBUG/INFO/WARN/ERROR 四个级别,全部插入成功,MySQL 自动分配递增的 log_id
- 查询最近 24 小时的 ERROR 日志返回 2 条记录（"连接超时，重试中" 和 "数据库查询失败：权限不足"）,时间均在当前时刻,符合筛选条件

---

## 任务三：商品库存管理（PostgreSQL）

### 实验要求

- 在 PostgreSQL 中创建 `products` 表
- 字段：商品ID、名称、库存数量、最后更新时间
- 实现库存扣减功能（购买商品后更新库存）
- 编写查询库存量低于 10 件的商品功能

### 实现要点

连接 PostgreSQL 后先 `DROP TABLE IF EXISTS products CASCADE` 清理旧表，再创建新表：`id SERIAL PRIMARY KEY`、`name VARCHAR(100) NOT NULL UNIQUE`（保证商品名称唯一）、`stock INT NOT NULL DEFAULT 0`、`updated_at TIMESTAMP DEFAULT NOW()`。

插入 5 件测试商品（笔记本库存5、鼠标库存20、键盘库存8、显示器库存3、耳机库存12）。

库存扣减使用 `UPDATE products SET stock = stock - $1, updated_at = NOW() WHERE name = $2 AND stock >= $1`,通过 `WHERE stock >= $1` 保证不会出现负库存,`rows_affected() == 0` 表示库存不足或商品不存在。演示了两个场景:成功扣减（笔记本 5→2）和库存不足被拒绝（显示器仅3件,尝试购买5件）。

低库存查询使用 `WHERE stock < 10 ORDER BY stock ASC`。

### 运行结果

[运行任务三-PostgreSQL建表与插入商品]

[运行任务三-库存扣减]

[运行任务三-低库存查询]

### 结果分析

- products 表创建成功,name 字段的 UNIQUE 约束确保了商品名称不重复
- 5 件商品全部插入成功
- 笔记本扣减 3 件后剩余 2 件,库存扣减成功,`updated_at` 自动更新为操作时间
- 显示器只有 3 件库存,尝试购买 5 件时 `rows_affected() == 0`,正确拒绝扣减
- 查询库存低于 10 件的结果:笔记本(2)、显示器(3)、键盘(8),共 3 件,`ORDER BY stock ASC` 按库存升序排列

---

## 任务四：数据统计报表（MySQL）

### 实验要求

- 基于订单表生成每日销售报表
- 统计：订单总数、销售总额、平均客单价
- 使用数据库视图存储统计逻辑
- 实现按日期范围查询统计结果

### 实现要点

在 MySQL 中创建 `orders` 表：`order_id INT AUTO_INCREMENT PRIMARY KEY`、`amount DECIMAL(10,2) NOT NULL`、`created_at DATETIME DEFAULT CURRENT_TIMESTAMP`。

插入 8 条测试订单,金额在 ¥79~¥599 之间,created_at 分布在执行当天、前一天、前两天（使用 `NOW()`、`NOW() - INTERVAL 1 DAY`、`NOW() - INTERVAL 2 DAY`）。

创建视图 `daily_sales_report`,使用 `DATE(created_at)` 按天分组,统计 `COUNT(*)`（订单总数）、`SUM(amount)`（销售总额）、`AVG(amount)`（平均客单价）,并将 DECIMAL 类型 CAST 为 DOUBLE 以满足 Rust f64 映射。

按日期范围查询使用 `WHERE order_date BETWEEN ? AND ?` 绑定起止日期参数。

### 运行结果

[运行任务四-MySQL建表与订单插入]

[运行任务四-视图与销售报表查询]

### 结果分析

- orders 表创建成功,8 条测试订单覆盖 3 个日期（当天 3 单、前一天 2 单、前两天 3 单）
- `CREATE OR REPLACE VIEW daily_sales_report` 成功创建视图,按日聚合订单数据
- 查询 2026-06-18 至 2026-06-21 的销售报表:
  - 06-19：3 单,总额 ¥1027.00,均客单价 ¥342.33
  - 06-20：2 单,总额 ¥579.00,均客单价 ¥289.50
  - 06-21：3 单,总额 ¥588.40,均客单价 ¥196.13
- 视图封装了聚合逻辑,外部查询只需简单的 `BETWEEN` 条件即可按任意日期范围获取统计数据
