
DROP SCHEMA IF EXISTS wepo CASCADE;

-- 创建 wepo schema
CREATE SCHEMA wepo;

-- 备注
COMMENT ON SCHEMA wepo
    IS 'wepo schemas';

-- 创建拓展
CREATE EXTENSION IF NOT EXISTS "pgcrypto"
    WITH SCHEMA wepo CASCADE;


-- 用户表 (id自增)
CREATE TABLE IF NOT EXISTS wepo.users
(
    id serial NOT NULL,
    nick character varying(15) NOT NULL UNIQUE,
    pwd text,
    _salt text NOT NULL,
    create_time DATE NOT NULL DEFAULT CURRENT_DATE,
    CONSTRAINT users_pkey PRIMARY KEY (id)
);

-- post表 sender外键约束，删除账号时自动删除其所有po文
CREATE TABLE IF NOT EXISTS wepo.posts
(
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    sender integer NOT NULL REFERENCES wepo.users(id) ON DELETE CASCADE,
    content text NOT NULL,
    create_time timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    likes integer NOT NULL DEFAULT 0,
    CONSTRAINT posts_pkey PRIMARY KEY (id)
);

-- post like表
CREATE TABLE IF NOT EXISTS wepo.post_likes
(
    post_id uuid NOT NULL REFERENCES wepo.posts(id) on DELETE CASCADE,
    user_id integer NOT NULL REFERENCES wepo.users(id) on DELETE CASCADE,
    PRIMARY KEY (post_id, user_id)
);