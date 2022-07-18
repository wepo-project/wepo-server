CREATE OR REPLACE FUNCTION post_comments_update_func() RETURNS TRIGGER AS
$$
    BEGIN
        IF (TG_OP = 'DELETE') THEN
            IF OLD.extends IS NOT NULL THEN
                UPDATE main.posts SET comments = comments - 1 WHERE id = OLD.extends;
            END IF;
        ELSEIF (TG_OP = 'UPDATE') THEN
            IF OLD.extends IS DISTINCT FROM NEW.extends THEN
                IF NEW.extends IS NULL THEN 
                    UPDATE main.posts SET comments = comments - 1 WHERE id = OLD.extends;
                ELSE
                    UPDATE main.posts SET comments = comments + 1 WHERE id = NEW.extends;
                END IF;
            END IF;
        ELSEIF (TG_OP = 'INSERT') THEN
            IF NEW.extends IS NOT NULL THEN
                UPDATE main.posts SET comments = comments + 1 WHERE id = NEW.extends;
            END IF;
        END IF;
        RETURN NULL;
    END
$$
LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER post_comments_updator
AFTER INSERT OR UPDATE OR DELETE
ON main.posts
FOR EACH ROW
EXECUTE PROCEDURE post_comments_update_func();