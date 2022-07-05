SELECT * FROM wepo.posts
WHERE sender = $1
ORDER BY create_time DESC
LIMIT $2 OFFSET $3;