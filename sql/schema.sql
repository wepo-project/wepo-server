
DROP SCHEMA IF EXISTS wepo CASCADE;
-- 创建 wepo schema
CREATE SCHEMA wepo;
COMMENT ON SCHEMA wepo
    IS 'wepo schemas';

-- 用户表 (id自增)
CREATE TABLE IF NOT EXISTS wepo.users
(
    id serial NOT NULL,
    nick character varying(15) NOT NULL UNIQUE,
    pwd character varying(20),
    _salt text NOT NULL,
    create_time DATE NOT NULL DEFAULT CURRENT_DATE,
    CONSTRAINT users_pkey PRIMARY KEY (id)
);