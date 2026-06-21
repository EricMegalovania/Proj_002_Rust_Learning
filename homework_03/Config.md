## MySQL

```bash
mysql -u root -p
```

```sql
-- 1. 创建数据库
CREATE DATABASE rust_hw03 CHARACTER SET utf8mb4;

-- 2. 创建新用户（把 '你的密码' 换成你设的密码）
CREATE USER 'user_rshw03'@'localhost' IDENTIFIED BY '123123';

-- 3. 把这个数据库的所有权限授予新用户
GRANT ALL PRIVILEGES ON rust_hw03.* TO 'user_rshw03'@'localhost';
FLUSH PRIVILEGES;

-- 4. 退出
EXIT;
```

```bash
mysql -u user_rshw03 -p rust_hw03  # 验证能否登录
```

## PostgreSQL

```bash
psql -U postgres
```

```sql
-- 1. 创建数据库
CREATE DATABASE rust_hw03;

-- 2. 创建新用户（输入你要设的密码）
CREATE USER user_rshw03 WITH PASSWORD '123123';

-- 3. 把数据库的所有者改为新用户
ALTER DATABASE rust_hw03 OWNER TO user_rshw03;

-- 4. 授予该数据库的所有权限（pg15+ 用这句）
GRANT ALL PRIVILEGES ON DATABASE rust_hw03 TO user_rshw03;

-- 5. 让新用户能操作 public schema 中的表
\c rust_hw03
GRANT ALL ON SCHEMA public TO user_rshw03;

-- 6. 退出
\q
```

```bash
psql -U user_rshw03 -d rust_hw03  # 验证能否登录
```

## 总结

| 配置项 | MySQL       | PostgreSQL  |
| ------ | ----------- | ----------- |
| Host   | 127.0.0.1   | 127.0.0.1   |
| Port   | 3306        | 5432        |
| 用户   | user_rshw03 | user_rshw03 |
| 密码   | 123123      | 123123      |
| 数据库 | rust_hw03   | rust_hw03   |
