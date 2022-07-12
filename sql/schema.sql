
DROP SCHEMA IF EXISTS main CASCADE;

-- 创建 main schema
CREATE SCHEMA main;

-- 备注
COMMENT ON SCHEMA main
    IS 'main schemas';

-- 创建拓展
CREATE EXTENSION IF NOT EXISTS "pgcrypto"
    WITH SCHEMA main CASCADE;


-- 用户表 (id自增)
CREATE TABLE IF NOT EXISTS main.users
(
    -- ID 自增
    id serial NOT NULL,
    -- 昵称
    nick character varying(15) NOT NULL UNIQUE,
    -- 密码 （加密后的）
    pwd text,
    -- 密码盐
    _salt text NOT NULL,
    -- 创建时间
    create_time DATE NOT NULL DEFAULT CURRENT_DATE,
    CONSTRAINT users_pkey PRIMARY KEY (id)
);

-- post表 sender外键约束，删除账号时自动删除其所有po文
CREATE TABLE IF NOT EXISTS main.posts
(
    -- id 雪花id
    id bigint NOT NULL,
    -- 发送者 外键
    sender integer NOT NULL REFERENCES main.users(id) ON DELETE CASCADE,
    -- 内容
    content text NOT NULL,
    -- 创建时间
    create_time timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- 点赞数量
    likes bigint NOT NULL DEFAULT 0,
    -- 评论数量
    comments bigint NOT NULL DEFAULT 0,
    -- 继承（评论）哪条po文
    extends bigint REFERENCES main.posts(id),
    CONSTRAINT posts_pkey PRIMARY KEY (id)
);

-- -- post like表
-- CREATE TABLE IF NOT EXISTS main.post_likes
-- (
--     -- po文id
--     post_id bigint NOT NULL REFERENCES main.posts(id) on DELETE CASCADE,
--     -- 用户id
--     user_id integer NOT NULL REFERENCES main.users(id) on DELETE CASCADE,
--     -- 两者为主键
--     PRIMARY KEY (post_id, user_id)
-- );