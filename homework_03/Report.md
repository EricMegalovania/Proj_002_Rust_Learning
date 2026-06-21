Rust 第三次实验 — 结果分析

================================================================================

任务一：用户注册系统（SQLite）

【实验要求】

1. 创建 SQLite 数据库 users.db
2. 建立 users 表（id 主键、username 唯一、password、created_at 时间戳）
3. 编写 Rust 程序实现用户注册功能，插入测试数据（至少 5 条）
4. 实现按用户名查询用户信息的功能

【面向对象设计】

数据类 User：包含 id(i64)、username(String)、password(String)、created_at(String) 四个字段，对应 users 表的全部列。

仓储类 UserRepo：持有 SqlitePool 连接池，对外暴露以下方法：
  new()          — 建立 SQLite 连接并返回 UserRepo 实例
  create_table() — 执行 CREATE TABLE IF NOT EXISTS，确保 users 表存在
  insert()       — 执行 INSERT OR IGNORE，返回 bool 表示是否新增
  find_by_username() — 按用户名查询，返回 Option<User>

run() 函数内编写了 4 个测试用例：连接建表、插入 5 条用户、按用户名查询、查询不存在的用户。

【运行结果】

[运行任务一-建表与插入用户]

[运行任务一-查询用户]

【结果分析】

1. UserRepo::new() 成功连接 SQLite，users.db 文件在项目根目录自动生成。
2. create_table() 建表语句正确执行，id 为自增主键，username 设 UNIQUE 约束。
3. insert() 插入 5 条测试用户（alice/bob/charlie/diana/eve），全部插入成功；
   重复运行时 INSERT OR IGNORE 机制正确跳过已存在记录，返回 false。
4. find_by_username("alice") 成功返回 User 实例，包含 id、username、password、created_at 四个字段，created_at 采用本地时间格式。
5. find_by_username("nobody") 正确返回 None，验证了查询不存在用户时的空值处理。

================================================================================

任务二：日志记录系统（MySQL）

【实验要求】

1. 连接本地 MySQL 数据库创建 app_logs 表
2. 字段包含：log_id（自增）、log_level（ENUM）、content、created_at
3. 编写日志写入函数，支持插入不同级别的日志
4. 实现查询最近 24 小时的 ERROR 级别日志

【面向对象设计】

枚举类 LogLevel：定义 Debug、Info、Warn、Error 四个变体，提供 as_str() 方法将枚举值转换为对应的 SQL ENUM 字符串("DEBUG"/"INFO"/"WARN"/"ERROR")。

数据类 AppLog：包含 log_id(i32)、log_level(String)、content(String)、created_at(NaiveDateTime) 四个字段。实现了 Display trait，用于格式化输出日志行。

仓储类 LogRepo：持有 MySqlPool 连接池，对外暴露以下方法：
  new()             — 建立 MySQL 连接并返回 LogRepo 实例
  create_table()    — 创建 app_logs 表（含 ENUM 类型定义）
  truncate()        — 清空表数据，保证每次运行输出一致
  insert()          — 接收 LogLevel 枚举和内容字符串，写入日志
  find_errors_24h() — 查询最近24小时的 ERROR 日志，返回 Vec<AppLog>

run() 函数内编写了 4 个测试用例：连接建表、清空旧数据、插入各级别日志、查询 ERROR 日志。

【运行结果】

[运行任务二-建表与日志写入]

[运行任务二-查询ERROR日志]

【结果分析】

1. LogRepo::new() 成功连接本地 MySQL 数据库 rust_hw03。
2. create_table() 创建 app_logs 表，log_level 字段使用 ENUM('DEBUG','INFO','WARN','ERROR') 约束合法性；log_id 自增主键，created_at 默认当前时间戳。
3. truncate() 每次运行前清空数据，确保测试输出可重复。
4. insert() 依次插入 6 条日志，覆盖全部四个级别（INFO×2、DEBUG×1、WARN×1、ERROR×2），LogLevel 枚举通过 as_str() 转换为数据库 ENUM 值，全部插入成功并返回自增的 log_id。
5. find_errors_24h() 过滤 log_level='ERROR' 且 created_at 在最近 24 小时内，返回 2 条 ERROR 日志（"连接超时，重试中"、"数据库查询失败：权限不足"），按时间倒序排列。AppLog 的 Display 实现格式化输出 [log_id] 时间 | 级别 | 内容。

================================================================================

任务三：商品库存管理（PostgreSQL）

【实验要求】

1. 在 PostgreSQL 中创建 products 表
2. 字段：商品ID、名称、库存数量、最后更新时间
3. 实现库存扣减功能（购买商品后更新库存）
4. 编写查询库存量低于 10 件的商品功能

【面向对象设计】

数据类 Product：包含 id(i32)、name(String)、stock(i32)、updated_at(NaiveDateTime) 四个字段，对应 products 表的全部列。

仓储类 ProductRepo：持有 PgPool 连接池，对外暴露以下方法：
  new()           — 建立 PostgreSQL 连接并返回 ProductRepo 实例
  create_table()  — 先 DROP 旧表再 CREATE 新表，name 设 UNIQUE 约束
  insert()        — 插入商品，ON CONFLICT (name) DO NOTHING 防重复
  deduct_stock()  — 扣减库存，WHERE stock >= quantity 防超卖，返回 bool
  get_stock()     — 查询指定商品的当前库存量
  find_low_stock() — 查询库存低于阈值的商品，按库存升序排列

run() 函数内编写了 5 个测试用例：连接建表、插入商品、正常扣减、库存不足拒绝、低库存查询。

【运行结果】

[运行任务三-建表与插入商品]

[运行任务三-库存扣减]

[运行任务三-低库存查询]

【结果分析】

1. ProductRepo::new() 成功连接本地 PostgreSQL 数据库 rust_hw03。
2. create_table() 先 DROP TABLE IF EXISTS products CASCADE 清理旧表，再重建。id 为 SERIAL 自增主键，name 设 VARCHAR(100) UNIQUE 约束，stock 默认 0，updated_at 默认 NOW()。
3. insert() 插入 5 件商品（笔记本×5、鼠标×20、键盘×8、显示器×3、耳机×12），全部成功。ON CONFLICT (name) DO NOTHING 确保重复运行不会报唯一键冲突。
4. deduct_stock("笔记本", 3) 扣减成功：5→2，updated_at 自动更新为操作时刻。deduct_stock("显示器", 5) 因为库存仅 3 件，WHERE stock >= 5 条件不满足，rows_affected 为 0，返回 false，正确拒绝超卖。
5. get_stock("显示器") 在扣减失败后确认库存仍为 3，未被错误修改。
6. find_low_stock(10) 返回库存低于 10 的商品：笔记本(2)、显示器(3)、键盘(8)，按 stock ASC 升序排列，共 3 件。

================================================================================

任务四：数据统计报表（MySQL）

【实验要求】

1. 基于订单表生成每日销售报表
2. 统计：订单总数、销售总额、平均客单价
3. 使用数据库视图存储统计逻辑
4. 实现按日期范围查询统计结果

【面向对象设计】

数据类 Order：包含 order_id(i32)、amount(f64)、created_at(NaiveDateTime) 三个字段。该类在代码中作为数据模型定义，实际查询通过视图完成。

数据类 DailySalesReport：包含 order_date(NaiveDate)、order_count(i64)、total_sales(f64)、avg_order_value(f64) 四个字段，对应视图 daily_sales_report 的输出列。

仓储类 OrderRepo：持有 MySqlPool 连接池，对外暴露以下方法：
  new()                  — 建立 MySQL 连接并返回 OrderRepo 实例
  create_table()         — 创建 orders 表
  truncate()             — 清空表数据
  insert_with_days_ago() — 插入订单，days_ago 参数指定订单日期（0=当天，1=昨天...）
  create_report_view()   — 创建/替换视图 daily_sales_report，封装日聚合逻辑
  query_report()         — 按日期范围查询视图，返回 Vec<DailySalesReport>

run() 函数内编写了 5 个测试用例：连接建表、清空旧数据、插入跨天订单、创建视图、按日期范围查询报表。

【运行结果】

[运行任务四-建表与订单插入]

[运行任务四-视图与报表查询]

【结果分析】

1. OrderRepo::new() 成功连接本地 MySQL 数据库 rust_hw03。
2. create_table() 创建 orders 表：order_id 自增主键，amount 使用 DECIMAL(10,2) 存储金额，created_at 默认当前时间。
3. truncate() 确保每次运行输出一致。
4. insert_with_days_ago() 通过 SQL 表达式 NOW() - INTERVAL ? DAY 绑定 days_ago 参数，插入 8 条测试订单，金额 ¥79.00~¥599.00，分布在当天(0天前×3)、昨天(1天前×2)、前天(2天前×3)，全部成功并返回自增 order_id。
5. create_report_view() 使用 CREATE OR REPLACE VIEW 创建视图 daily_sales_report：按 DATE(created_at) 分组，统计 COUNT(*) 为订单数、SUM(amount) 为销售总额、AVG(amount) 为平均客单价，DECIMAL 结果 CAST 为 DOUBLE 以匹配 Rust f64 类型。
6. query_report("2026-06-18", "2026-06-21") 查询指定日期范围的报表，返回 3 天数据：
   06-19: 3 单, 总额 ¥1027.00, 均客单价 ¥342.33
   06-20: 2 单, 总额 ¥579.00,  均客单价 ¥289.50
   06-21: 3 单, 总额 ¥588.40,  均客单价 ¥196.13
   视图封装了全部聚合逻辑，外部查询仅需简单的 BETWEEN 条件即可灵活指定日期范围。

================================================================================

总结

本次实验覆盖 SQLite、MySQL、PostgreSQL 三种主流数据库，使用 Rust 的 sqlx 异步库统一操作。每个任务均采用面向对象设计：定义数据类承载查询结果，定义仓储类封装数据库操作，将连接管理、DDL、DML、查询逻辑内聚在 impl 块中。run() 函数内以测试用例的形式依次验证各项功能，输出清晰、可重复运行。
