SELECT 
    p.id, 
    p.create_time, 
    p.content, 
    p.likes, 
    p.hates,
    p.comments,
    u.nick AS sender_nick, 
    u.id AS sender_id
FROM main.posts AS p
LEFT JOIN main.users u ON u.id = p.sender
WHERE p.extends = $1
ORDER BY p.create_time DESC
LIMIT $2 OFFSET $3;