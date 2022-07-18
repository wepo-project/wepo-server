SELECT * FROM main.notices
WHERE addressee_id = $1
ORDER BY p.create_time DESC
LIMIT $2 OFFSET $3;