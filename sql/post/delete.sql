DELETE FROM main.posts 
WHERE id = $1 AND sender = $2 
RETURNING id, extends;