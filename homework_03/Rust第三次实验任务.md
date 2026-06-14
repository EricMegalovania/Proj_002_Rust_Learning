## 任务一

用户注册系统（SQLite）

- 创建一个SQLite 数据库users.db
- 建立users 表（id 主键、username 唯一、password、created_at
时间戳）
- 编写Rust 程序实现用户注册功能，插入测试数据（至少5 条）
- 实现按用户名查询用户信息的功能

## 任务二

日志记录系统（MySQL）

- 连接本地MySQL 数据库创建app_logs 表
- 字段包含：log_id（自增）、log_level（ENUM）、content、
created_at
- 编写日志写入函数，支持插入不同级别的日志
- 实现查询最近24 小时的ERROR 级别日志

## 任务三

商品库存管理（PostgreSQL）

- 在PostgreSQL 中创建products 表
- 字段：商品ID、名称、库存数量、最后更新时间
- 实现库存扣减功能（购买商品后更新库存）
- 编写查询库存量低于10 件的商品功能

## 任务四

数据统计报表（MySQL）

- 基于订单表生成每日销售报表
- 统计：订单总数、销售总额、平均客单价
- 使用数据库视图存储统计逻辑
- 实现按日期范围查询统计结果