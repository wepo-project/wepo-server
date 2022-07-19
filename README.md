# Wepo 服务器

- 安装 [PostgreSQL](https://www.postgresql.org/download/) 数据库
- 安装 [Redis](https://redis.io/download/) 数据库
- 安装 [Rust](https://www.rust-lang.org/tools/install) 编程语言

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

```sh
# 创建 schema 和 table
psql -f sql/schema.sql wepo -U postgres

# 创建 trigger 和 function
psql -f sql/trigger.sql wepo -U postgres
```

### 4. 赋予权限

```sh
# 切换到 wepo 数据库
\c wepo
```

```sql
GRANT ALL PRIVILEGES ON SCHEMA main TO ww;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA main to ww;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA main to ww;

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

```sh
cargo run
```

自动重新加载开发服务器 (需要先安装 [cargo-watch](https://github.com/watchexec/cargo-watch) )

```sh
cargo watch -x 'run'
```

### 7. 运行客户端（精简测试版）

```sh
cd test-client
# 安装依赖 或者使用npm、pnpm
yarn install
# 运行
yarn dev
```