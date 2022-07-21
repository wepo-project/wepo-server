UPDATE main.posts
SET status = $1;
WHERE id = $2 AND sender = $3

RETURNING id, extends;