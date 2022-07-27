SELECT
    n.id,
    -- n.read,
    n.create_time,
    n.sender_object,
    u.id AS sender_id,
    u.nick AS sender_nick,
    u.avatar_url AS sender_avatar_url
FROM main.notices as n
LEFT JOIN main.users as u ON u.id = n.sender
WHERE notice_type = $1 and addressee_id = $2
ORDER BY n.create_time DESC
LIMIT $3 OFFSET $4;