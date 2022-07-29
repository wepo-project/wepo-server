
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
    -- 头像
    avatar_url character varying(256),
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
    likes integer NOT NULL DEFAULT 0,
    -- 评论数量
    comments integer NOT NULL DEFAULT 0,
    -- 讨厌数量
    hates integer NOT NULL DEFAULT 0,
    -- 继承（评论） 如果原文删除，则查找不到
    extends bigint,
    -- 主键约束
    CONSTRAINT posts_pkey PRIMARY KEY (id)
);

-- 通知
CREATE TABLE IF NOT EXISTS main.notices
(
    -- id
    id bigserial NOT NULL,
    -- 发送者
    sender integer NOT NULL,
    -- 通知类型
    notice_type smallint NOT NULL,
    -- 通知主体的id ， 如果是评论，则为评论id
    sender_object text NOT NULL,
    -- 接收者
    addressee_id integer NOT NULL REFERENCES main.users(id) ON DELETE CASCADE,
    -- 创建时间
    create_time timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- -- 是否已读
    -- read boolean NOT NULL DEFAULT FALSE,
    -- 主键约束
    CONSTRAINT notices_pkey PRIMARY KEY (id),
    -- 唯一约束
    UNIQUE (sender, notice_type, sender_object)
);

-- 好友关系表
CREATE TABLE IF NOT EXISTS main.friendship
(
    -- 请求者
    requester_id integer NOT NULL REFERENCES main.users(id) ON DELETE CASCADE,
    -- 接收者
    addressee_id integer NOT NULL REFERENCES main.users(id) ON DELETE CASCADE,
    -- 创建日期
    create_time DATE NOT NULL DEFAULT CURRENT_DATE,
    -- 主键约束
    CONSTRAINT friendship_pkey PRIMARY KEY (requester_id, addressee_id)
)