SELECT
    n.id,
    -- n.read,
    n.create_time,
    n.sender_object,
    u.id AS sender_id,
    u.nick AS sender_nick,
    u.avatar_url AS sender_avatar_url,
    p.content::varchar(50) AS content,
    p.extends AS origin_id,
    p2.content::varchar(20) AS origin,
    p2.create_time AS origin_create_time
FROM main.notices as n
LEFT JOIN main.users as u ON u.id = n.sender
LEFT JOIN main.posts as p ON p.id = CAST(n.sender_object as bigint)
LEFT JOIN main.posts as p2 ON p.extends = p2.id
WHERE notice_type = $1 and addressee_id = $2
ORDER BY n.create_time DESC
LIMIT $3 OFFSET $4;