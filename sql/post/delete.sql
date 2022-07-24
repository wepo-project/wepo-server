DELETE FROM main.posts
WHERE id = $2 AND sender = $3
RETURNING id, extends;