WITH rows AS (
    INSERT INTO main.posts
        (id, sender, content, extends)
    VALUES
        ($1, $2, $3, $4)
    RETURNING *
)
SELECT 
    r.id,
    r.extends,
    old.sender as receiver
from main.posts as old, rows as r
WHERE old.id = r.extends