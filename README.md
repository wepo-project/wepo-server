# Wepo 服务器

> 1、安装 [PostgreSQL](https://www.postgresql.org/download/) 数据库
> 
> 2、安装 [Rust](https://www.rust-lang.org/tools/install) 编程语言

## 起步

> 假设用户名/密码为: ww/ww

### 1. 创建用户

```sql
CREATE USER ww WITH PASSWORD 'ww';
```

### 2. 创建数据库

```sql
CREATE DATABASE wepo OWNER ww;
```

### 3. 初始化数据库

```shell
psql -f sql/schema.sql wepo -U postgres
```
> postgres: 超级管理员，如果不是则替换

### 4. 赋予权限

**切换到 wepo 数据库**

```shell
\c wepo
```

```sql
GRANT ALL PRIVILEGES ON SCHEMA wepo TO ww;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA wepo to ww;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA wepo to ww;
```

### 5. 创建 `.env` 文件

```ini
SERVER_ADDR=127.0.0.1:8080
REDIS_ADDR=127.0.0.1:6379
PG.USER=ww
PG.PASSWORD=ww
PG.HOST=127.0.0.1
PG.PORT=5432
PG.DBNAME=wepo
PG.POOL.MAX_SIZE=16
```

### 6. 运行服务器

直接运行

```shell
cargo run
```

自动重新加载开发服务器 ([cargo-watch](https://github.com/watchexec/cargo-watch))

```shell
cargo watch -x 'run'
```