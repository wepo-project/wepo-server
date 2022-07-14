SELECT p.id, u.sender_nick, u.sender_id, p.create_time, p.content, p.likes, p.comments
FROM main.posts AS p
LEFT JOIN main.users u ON u.id = p.sender
WHERE p.extends = $1
ORDER BY p.create_time DESC
LIMIT $2 OFFSET $3;