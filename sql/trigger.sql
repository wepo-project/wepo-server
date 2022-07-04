CREATE OR REPLACE FUNCTION post_like_add() RETURNS TRIGGER AS
$$
    BEGIN
        UPDATE wepo.posts SET likes = likes + 1 WHERE id = NEW.post_id;
        RETURN NEW;
    END
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION post_like_sub() RETURNS TRIGGER AS
$$
    BEGIN
        UPDATE wepo.posts SET likes = likes - 1 WHERE id = OLD.post_id;
        RETURN OLD;
    END
$$
LANGUAGE plpgsql;

-- DROP TRIGGER IF EXISTS post_like_insert;
-- DROP TRIGGER IF EXISTS post_like_delete;

CREATE TRIGGER post_like_insert AFTER INSERT ON wepo.post_likes
    FOR EACH ROW EXECUTE PROCEDURE post_like_add();

CREATE TRIGGER post_like_delete AFTER DELETE ON wepo.post_likes
    FOR EACH ROW EXECUTE PROCEDURE post_like_sub();